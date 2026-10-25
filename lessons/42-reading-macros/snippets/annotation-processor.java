import java.net.URI;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Set;
import javax.annotation.processing.AbstractProcessor;
import javax.annotation.processing.RoundEnvironment;
import javax.annotation.processing.SupportedAnnotationTypes;
import javax.lang.model.SourceVersion;
import javax.lang.model.element.Element;
import javax.lang.model.element.TypeElement;
import javax.tools.JavaFileObject;
import javax.tools.SimpleJavaFileObject;
import javax.tools.ToolProvider;

public class Main {
    // A processor runs inside javac, reads the annotated class and writes a NEW source file.
    @SupportedAnnotationTypes("Marked")
    static class Processor extends AbstractProcessor {
        @Override
        public SourceVersion getSupportedSourceVersion() {
            return SourceVersion.latestSupported();
        }

        @Override
        public boolean process(Set<? extends TypeElement> annotations, RoundEnvironment round) {
            for (TypeElement a : annotations) {
                for (Element e : round.getElementsAnnotatedWith(a)) {
                    String name = e.getSimpleName() + "Info";
                    try (var out = processingEnv.getFiler().createSourceFile(name).openWriter()) {
                        out.write("class " + name + " { static String kind = \"" + e.getKind() + "\"; }\n");
                    } catch (Exception ex) {
                        throw new RuntimeException(ex);
                    }
                }
            }
            return false;
        }
    }

    // Everything below is only a harness that calls javac from Java.
    public static void main(String[] args) throws Exception {
        Path dir = Files.createTempDirectory("gen");
        var source = new SimpleJavaFileObject(URI.create("string:///Thing.java"), JavaFileObject.Kind.SOURCE) {
            @Override
            public CharSequence getCharContent(boolean ignoreErrors) {
                return "@interface Marked {}\n@Marked class Thing {}\n";
            }
        };
        var javac = ToolProvider.getSystemJavaCompiler();
        var task = javac.getTask(null, null, null, List.of("-proc:only", "-s", dir.toString()), null, List.of(source));
        task.setProcessors(List.of(new Processor()));
        System.out.println("javac: " + (task.call() ? "ok" : "failed"));
        Path generated = dir.resolve("ThingInfo.java");
        System.out.print("ThingInfo.java on disk: " + Files.readString(generated));
        Files.delete(generated);
        Files.delete(dir);
    }
}
