import java.io.*;
import java.net.URI;
import java.nio.file.*;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;
import java.util.Objects;
import java.util.stream.Stream;

/*
File objCopy = new File("./java/lang/" + classname + ".class");
            objCopy.createNewFile();
            FileWriter writer = new FileWriter("./java/lang/" + classname + ".class");
            byte[] jlo = Files.readAllBytes(fs.getPath("modules", "java.base",
                    "./java/lang/" + classname + ".class"));
            writer.write(Arrays.toString(jlo));

 private static void parsefiles(Iterator<Path> piter) {
        try {
            while (piter.hasNext()) {
                Path p = piter.next();
                File objCopy = new File("." + p.toString());
                if(!objCopy.getParentFile().exists()) {
                    objCopy.getParentFile().mkdirs();
                }
                objCopy.createNewFile();
                FileWriter writer = new FileWriter("." + p.toString());
                byte[] jlo = Files.readAllBytes(p);
                writer.write(Arrays.toString(jlo));
            }
        }
        catch (IOException e) {
            System.out.println(e.getMessage());
            System.out.println(Arrays.toString(e.getStackTrace()));
        }
    }

 */

public class Main {
    private static void parsefiles(FileSystem fs, Iterator<Path> piter) {
        try {
            while (piter.hasNext()) {
                Path p = piter.next();
                File objCopy = new File("." + p.toString());
                if(!objCopy.getParentFile().exists()) {
                    objCopy.getParentFile().mkdirs();
                }
                try {
                    Stream<Path> subfiles = Files.list(p);
                    parsefiles(fs, subfiles.iterator());
                }
                catch(NotDirectoryException e) {
                    String[] subpaths = objCopy.getPath().split("/");
                    if (Objects.equals(subpaths[subpaths.length - 1], "module-info.class")) {
                        continue;
                    }
                    if (Objects.equals(subpaths[2].substring(0, 3), "jdk")) {
                        continue;
                    }
                    objCopy.createNewFile();
                    FileOutputStream writer = new FileOutputStream(objCopy);
                    byte[] jlo = Files.readAllBytes(p);
                    Files.write(objCopy.toPath(), jlo);
                    writer.close();
                    System.out.println(objCopy.getPath() + ", size: " + jlo.length);
                }

            }
        }
        catch (IOException e) {
            //System.out.println(e.getMessage());
            //System.out.println(Arrays.toString(e.getStackTrace()));
        }
    }

    public static void main(String[] args) {
        FileSystem fs = FileSystems.getFileSystem(URI.create("jrt:/"));
        try {
            parsefiles(fs, Files.list(fs.getPath("/modules")).iterator());
        }
        catch (IOException e) {
            System.out.println(e.getMessage());
            System.out.println(Arrays.toString(e.getStackTrace()));
        }

    }
}