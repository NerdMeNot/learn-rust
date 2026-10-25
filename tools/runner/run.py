"""Compile and run programs inside the runner image.

    docker run -i --rm learnrust-runner < jobs.json > results.json
    docker run --rm learnrust-runner versions

A job is {"key", "lang", "mode", "file", "code"}. For each one this compiles the program
the way the page says (cargo, go, kotlinc, tsc, javac, dotnet, gcc, g++), runs it, and
reports what a reader would see in their terminal.

Programs run under a pseudo-terminal, not a pipe. Through a pipe, C's printf and
Python's print buffer their output, so a crash loses lines a terminal would have shown,
and stdout and stderr come out in the wrong order. A pty gives the terminal's behaviour.

Each program runs twice. If the two runs print different things, the result says so
(`stable: false`): a lesson that shows one run of a racy or hash-ordered program has to
say that the output varies, or fix the program.
"""

import json
import os
import pty
import re
import select
import shutil
import signal
import subprocess
import sys
import time

TIMEOUT = 30
JOB = '/tmp/job'
ANSI = re.compile(r'\x1b\[[0-9;?]*[A-Za-z]')


def sh(cmd, cwd, timeout=300, env=None):
    """Run a build step; return (ok, combined output)."""
    p = subprocess.run(cmd, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                       timeout=timeout, env={**os.environ, **(env or {})})
    return p.returncode == 0, p.stdout.decode('utf-8', 'replace')


def run_pty(cmd, cwd, env=None):
    """Run the program as a terminal would. Returns (exit code or None, output, timed out)."""
    master, slave = pty.openpty()
    start = time.time()
    p = subprocess.Popen(cmd, cwd=cwd, stdin=subprocess.DEVNULL, stdout=slave, stderr=slave,
                         env={**os.environ, 'PYTHONUNBUFFERED': '1', 'NO_COLOR': '1', 'NODE_DISABLE_COLORS': '1',
                              'TERM': 'dumb', **(env or {})},
                         start_new_session=True)
    os.close(slave)
    chunks, timed_out = [], False
    while True:
        if time.time() - start > TIMEOUT:
            os.killpg(p.pid, signal.SIGKILL)
            timed_out = True
        r, _, _ = select.select([master], [], [], 0.1)
        if r:
            try:
                data = os.read(master, 65536)
            except OSError:
                data = b''
            if not data:
                break
            chunks.append(data)
        elif p.poll() is not None:
            # drain whatever is left
            try:
                while True:
                    data = os.read(master, 65536)
                    if not data:
                        break
                    chunks.append(data)
            except OSError:
                pass
            break
    p.wait()
    os.close(master)
    out = b''.join(chunks).decode('utf-8', 'replace').replace('\r\n', '\n').replace('\r', '\n')
    # A terminal invites colour. The page shows plain text, so drop any escape codes.
    out = ANSI.sub('', out)
    return p.returncode, out, timed_out


def fresh(dirname):
    path = os.path.join(JOB, dirname)
    shutil.rmtree(path, ignore_errors=True)
    os.makedirs(path)
    return path


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, 'w') as f:
        f.write(text)


# ---------------------------------------------------------------- per language
# Each returns (compiled, compile output, argv to run, cwd, extra env).

def build_rust(job):
    d = fresh('rust')
    shutil.copy('/opt/templates/rust/Cargo.toml', d)
    write(f'{d}/src/main.rs', job['code'])
    ok, out = sh(['cargo', 'build', '-q'], d, env={'CARGO_TARGET_DIR': '/tmp/target-rust'})
    return ok, out, ['/tmp/target-rust/debug/main'], d, {}


def build_go(job):
    d = fresh('go')
    write(f'{d}/main.go', job['code'])
    flags = ['-race'] if job.get('mode') == 'race' else []
    ok, out = sh(['go', 'build', *flags, '-o', 'prog', 'main.go'], d,
                 env={'GOCACHE': '/tmp/gocache', 'GOFLAGS': '-mod=mod', 'CGO_ENABLED': '1' if flags else '0'})
    return ok, out, ['./prog'], d, {}


COROUTINES = '/opt/kotlinc/lib/kotlinx-coroutines-core-jvm.jar'  # hash-exempt
def build_kotlin(job):
    d = fresh('kotlin')
    write(f'{d}/Main.kt', job['code'])
    # kotlinx.coroutines ships beside the compiler but is not on its default classpath.  # hash-exempt
    # Only a program that imports it takes this path; every other program runs as before.  # hash-exempt
    # Lines marked hash-exempt stay out of the runner hash (website/scripts/extract.ts): they  # hash-exempt
    # cannot change what an existing program prints, so they must not re-run all of them.  # hash-exempt
    if 'kotlinx.coroutines' in job['code']:  # hash-exempt
        ok, out = sh(['kotlinc', 'Main.kt', '-cp', COROUTINES, '-include-runtime', '-d', 'main.jar'], d)  # hash-exempt
        return ok, out, ['java', '-cp', f'main.jar:{COROUTINES}', 'MainKt'], d, {}  # hash-exempt
    ok, out = sh(['kotlinc', 'Main.kt', '-include-runtime', '-d', 'main.jar'], d)
    return ok, out, ['java', '-jar', 'main.jar'], d, {}


def build_python(job):
    d = fresh('python')
    write(f'{d}/main.py', job['code'])
    return True, '', ['python3', 'main.py'], d, {}


def build_typescript(job):
    d = fresh('typescript')
    write(f'{d}/main.ts', job['code'])
    # TypeScript 6+ loads no @types by default, and emits ES modules unless told the
    # program is a Node script. These are the flags a Node project's tsconfig sets.
    ok, out = sh(['tsc', '--strict', '--module', 'nodenext', '--types', 'node', 'main.ts'], d)
    return ok, out, ['node', 'main.js'], d, {}


def build_java(job):
    d = fresh('java')
    write(f'{d}/Main.java', job['code'])
    ok, out = sh(['javac', 'Main.java'], d)
    return ok, out, ['java', 'Main'], d, {}


def build_csharp(job):
    # One project, reused: copying a restored project per job costs more than building.
    d = os.path.join(JOB, 'csharp')
    if not os.path.isdir(d):
        shutil.copytree('/opt/templates/csharp', d)
    write(f'{d}/Program.cs', job['code'])
    ok, out = sh(['dotnet', 'build', '--no-restore', '-v', 'q', '-nologo', '-clp:NoSummary'], d)
    # MSBuild prints each diagnostic with the project's absolute path, twice.
    lines, seen = [], set()
    for line in out.splitlines():
        line = line.replace(d + '/', '').strip()
        line = re.sub(r'\s*\[[^\]]*\.csproj\]$', '', line)
        if re.match(r'^Program\.cs\(\d+,\d+\): (error|warning)', line) and line not in seen:
            seen.add(line)
            lines.append(line)
    return ok, '\n'.join(lines) + ('\n' if lines else ''), ['dotnet', 'bin/Debug/net10.0/Program.dll'], d, {}


def build_c_like(job, compiler, std, src):
    d = fresh(job['lang'])
    write(f'{d}/{src}', job['code'])
    mode = job.get('mode')
    if mode == 'asan':
        flags = ['-fsanitize=address', '-g']
    elif mode == 'tsan':
        flags = ['-fsanitize=thread', '-g']
    else:
        flags = ['-Wall', '-Wextra']
    ok, out = sh([compiler, *std, *flags, src, '-o', 'main', *(['-lpthread'])], d)
    env = {'ASAN_OPTIONS': 'detect_leaks=0:color=never', 'TSAN_OPTIONS': 'halt_on_error=0:color=never'} if mode else {}
    # ThreadSanitizer refuses to start under the address-space randomisation newer
    # kernels use; running with it switched off is the documented workaround.
    argv = ['setarch', os.uname().machine, '-R', './main'] if mode == 'tsan' else ['./main']
    return ok, out, argv, d, env


BUILD = {
    'rust': build_rust,
    'go': build_go,
    'kotlin': build_kotlin,
    'python': build_python,
    'typescript': build_typescript,
    'java': build_java,
    'csharp': build_csharp,
    'c': lambda j: build_c_like(j, 'gcc', [], 'main.c'),
    'cpp': lambda j: build_c_like(j, 'g++', ['-std=c++20'], 'main.cpp'),
}


# ---------------------------------------------------------------- sessions
# A session is a whole project plus a list of shell commands run in order, the way a
# guide page walks through them: `cargo add`, `cargo run`, start a server and `curl` it.
#
# Cargo's progress lines ("Compiling serde v1.0.228") depend on what was already built
# and downloaded, so they are dropped unless a step says `raw`. A page about Cargo
# itself wants them; a page about serde wants the program's output.
CARGO_NOISE = re.compile(r'^\s+(Compiling|Finished|Running|Downloaded|Downloading|Updating|Locking|Adding|Blocking|Checking|Fresh|Packaging|Removing|Unpacking|Installing|Installed|Replacing)\b.*$|^\s*Downloaded \d+ crates?.*$', re.M)


def run_session(job):
    global TIMEOUT
    t0 = time.time()
    d = fresh('session')
    for path, text in job['files'].items():
        write(os.path.join(d, path), text)
    # Builds go to a shared, cached folder unless the page needs the real ./target/
    # (a page that runs ./target/release/<name> or shows what appears in target/).
    env = {} if job.get('local_target') else {'CARGO_TARGET_DIR': f"/tmp/target/{job.get('project', 'session')}"}
    background, steps = [], []
    try:
        for step in job['steps']:
            cwd = os.path.join(d, step.get('cwd', '.'))
            if step.get('background'):
                log = open(os.path.join(JOB, f'bg-{len(background)}.log'), 'w+')
                p = subprocess.Popen(['bash', '-c', step['cmd']], cwd=cwd, stdout=log, stderr=subprocess.STDOUT,
                                     env={**os.environ, **env}, start_new_session=True)
                background.append((p, log))
                ready = re.compile(step.get('ready', '.'))
                deadline = time.time() + step.get('timeout', 240)
                while time.time() < deadline:
                    log.seek(0)
                    text = log.read()
                    if ready.search(text) or p.poll() is not None:
                        break
                    time.sleep(0.2)
                log.seek(0)
                out = log.read()
                steps.append({'cmd': step['cmd'], 'exit': p.poll(), 'out': out if step.get('raw') else CARGO_NOISE.sub('', out).lstrip('\n')})
                continue
            TIMEOUT = step.get('timeout', 300)
            code, out, timed_out = run_pty(['bash', '-c', step['cmd']], cwd, env)
            if not step.get('raw'):
                out = re.sub(r'\n{3,}', '\n\n', CARGO_NOISE.sub('', out)).lstrip('\n')
            steps.append({'cmd': step['cmd'], 'exit': code, 'out': out, 'timed_out': timed_out})
    finally:
        for p, log in background:
            try:
                os.killpg(p.pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
            log.close()
        TIMEOUT = 30
    # Lock files the session created, so the repo can pin what it resolved.
    collected = {}
    for path in job.get('collect', []):
        full = os.path.join(d, path)
        if os.path.isfile(full):
            with open(full) as f:
                collected[path] = f.read()
    return {'key': job['key'], 'steps': steps, 'collected': collected, 'seconds': round(time.time() - t0, 2)}


def run_job(job):
    if job.get('kind') == 'session':
        return run_session(job)
    t0 = time.time()
    compiled, compile_out, argv, cwd, env = BUILD[job['lang']](job)
    result = {'key': job['key'], 'compiled': compiled, 'compile_out': compile_out}
    if compiled and job.get('run', True):
        code, out, timed_out = run_pty(argv, cwd, env)
        result.update(exit=code, out=out, timed_out=timed_out)
        if not timed_out and time.time() - t0 < 20:
            code2, out2, _ = run_pty(argv, cwd, env)
            result['stable'] = (code2 == code and out2 == out)
            if not result['stable']:
                result['out_2'] = out2
    result['seconds'] = round(time.time() - t0, 2)
    return result


def versions():
    def first(cmd):
        try:
            return subprocess.run(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT).stdout.decode().strip().splitlines()[0]
        except Exception as e:  # noqa: BLE001
            return f'unavailable: {e}'
    return {
        'rust': first(['rustc', '--version']),
        'go': first(['go', 'version']),
        'kotlin': first(['kotlinc', '-version']),
        'java': first(['java', '-version']),
        'python': first(['python3', '--version']),
        'typescript': first(['tsc', '--version']),
        'node': first(['node', '--version']),
        'csharp': '.NET ' + first(['dotnet', '--version']),
        'c': first(['gcc', '--version']),
        'cpp': first(['g++', '--version']),
    }


def main():
    if sys.argv[1:] == ['versions']:
        print(json.dumps(versions(), indent=1))
        return
    jobs = json.load(sys.stdin)
    os.makedirs(JOB, exist_ok=True)
    # tsc finds @types/node by walking up from the program's folder.
    if not os.path.exists('/tmp/node_modules'):
        os.symlink('/opt/node/lib/node_modules', '/tmp/node_modules')
    results = []
    for i, job in enumerate(jobs, 1):
        try:
            r = run_job(job)
        except Exception as e:  # noqa: BLE001
            r = {'key': job['key'], 'runner_error': repr(e)}
        results.append(r)
        print(f"[{i}/{len(jobs)}] {job['key']} {r.get('seconds', '?')}s", file=sys.stderr, flush=True)
    json.dump(results, sys.stdout)


if __name__ == '__main__':
    main()
