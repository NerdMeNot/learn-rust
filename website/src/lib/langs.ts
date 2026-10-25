// The nine languages on the site: display name, the file a reader would save the
// program as, the command they would run, and the colour token for tabs and chips.
// Shared by the extract script and the components so the two never disagree.

export type Lang = 'rust' | 'go' | 'kotlin' | 'python' | 'typescript' | 'java' | 'csharp' | 'c' | 'cpp'
export type Mode = 'race' | 'asan' | 'tsan'

export const OTHERS: Lang[] = ['go', 'kotlin', 'python', 'typescript', 'java', 'csharp', 'c', 'cpp']

export const LANG: Record<Lang, { name: string; file: string; cmd: string; color: string; hl: string; tagline?: string }> = {
  rust: { name: 'Rust', file: 'src/main.rs', cmd: '$ cargo run', color: 'var(--lr-rust)', hl: 'rust' },
  go: { name: 'Go', file: 'main.go', cmd: '$ go run main.go', color: 'var(--lr-go)', hl: 'go', tagline: 'goroutines, nil, if err != nil' },
  kotlin: { name: 'Kotlin', file: 'Main.kt', cmd: '$ kotlinc Main.kt -include-runtime -d main.jar && java -jar main.jar', color: 'var(--lr-kotlin)', hl: 'kotlin', tagline: 'null safety, data classes' },
  python: { name: 'Python', file: 'main.py', cmd: '$ python3 main.py', color: 'var(--lr-python)', hl: 'python', tagline: 'dynamic types, None, exceptions' },
  typescript: { name: 'TypeScript', file: 'main.ts', cmd: '$ tsc --strict --module nodenext --types node main.ts && node main.js', color: 'var(--lr-ts)', hl: 'typescript', tagline: 'undefined, async/await, npm' },
  java: { name: 'Java', file: 'Main.java', cmd: '$ javac Main.java && java Main', color: 'var(--lr-java)', hl: 'java', tagline: 'classes, null, exceptions' },
  csharp: { name: 'C#', file: 'Program.cs', cmd: '$ dotnet run', color: 'var(--lr-csharp)', hl: 'csharp', tagline: 'LINQ, async, nullable refs' },
  c: { name: 'C', file: 'main.c', cmd: '$ gcc -Wall -Wextra main.c -o main && ./main', color: 'var(--lr-c)', hl: 'c', tagline: 'malloc, free, raw pointers' },
  cpp: { name: 'C++', file: 'main.cpp', cmd: '$ g++ -std=c++20 -Wall -Wextra main.cpp -o main && ./main', color: 'var(--lr-cpp)', hl: 'cpp', tagline: 'RAII, templates, smart pointers' },
}

/** The command shown above an output, which depends on how the program was run. */
export function commandFor(lang: Lang, mode?: Mode): string {
  if (lang === 'go' && mode === 'race') return '$ go run -race main.go'
  if (lang === 'c' && mode === 'asan') return '$ gcc -fsanitize=address -g main.c -o main && ./main'
  if (lang === 'c' && mode === 'tsan') return '$ gcc -fsanitize=thread -g main.c -o main && ./main'
  if (lang === 'cpp' && mode === 'asan') return '$ g++ -std=c++20 -fsanitize=address -g main.cpp -o main && ./main'
  if (lang === 'cpp' && mode === 'tsan') return '$ g++ -std=c++20 -fsanitize=thread -g main.cpp -o main && ./main'
  return LANG[lang].cmd
}

/** What happened when a gotcha's code ran in another language. */
export type Status = 'runs' | 'wrong' | 'race' | 'crash' | 'refuses' | 'none'

export const STATUS: Record<Status, { cls: 'ok' | 'warn' | 'bad' | 'no'; tick: string; chip: string; phrase: string }> = {
  runs: { cls: 'ok', tick: '✓', chip: '✓ runs', phrase: 'it runs fine' },
  wrong: { cls: 'warn', tick: '!', chip: '! runs · wrong answer', phrase: 'it runs but gives a wrong answer' },
  race: { cls: 'warn', tick: '!', chip: '! runs · data race', phrase: 'it runs but has a data race' },
  crash: { cls: 'warn', tick: '!', chip: '! crashes at runtime', phrase: 'it crashes at runtime' },
  refuses: { cls: 'bad', tick: '✗', chip: '✗ refuses too', phrase: 'it refuses too' },
  none: { cls: 'no', tick: '–', chip: 'no equivalent', phrase: '' },
}

export type Verdict = 'fine-elsewhere' | 'broken-elsewhere' | 'rust-rule' | 'different-result'

export const VERDICT: Record<Verdict, { label: string; short: string; text: string }> = {
  'fine-elsewhere': {
    label: "Fine where you come from. Rust can't prove it's safe.",
    short: 'Fine elsewhere',
    text: "Your version really is fine. But the same shape of code breaks in other situations, and the compiler can't tell your case from those, so it refuses them all.",
  },
  'broken-elsewhere': {
    label: 'Broken where you come from. Rust catches it.',
    short: 'Broken elsewhere',
    text: 'Your version runs, but it is actually wrong: a null crash, a silently wrong answer, or a data race. In Rust the same mistake is a compile error.',
  },
  'rust-rule': {
    label: 'Just a Rust rule.',
    short: 'Rust rule',
    text: 'Nothing could go wrong here. Rust just has a different rule or default from the language you know. You learn it once and move on.',
  },
  'different-result': {
    label: 'Compiles in both. Different result.',
    short: 'Different result',
    text: "Both versions compile and run, but they don't do the same thing. Usually the other language quietly picks a default where Rust makes you say what you mean.",
  },
}

export const PARTS = [
  { id: 'syntax', label: 'Syntax' },
  { id: 'basics', label: 'Basics' },
  { id: 'ownership', label: 'Ownership' },
  { id: 'data', label: 'Data' },
  { id: 'collections', label: 'Working with collections' },
  { id: 'abstractions', label: 'Abstractions' },
  { id: 'smart-pointers', label: 'Smart pointers' },
  { id: 'concurrency', label: 'Concurrency' },
  { id: 'std', label: 'Standard library' },
  { id: 'further', label: 'Going further' },
] as const
