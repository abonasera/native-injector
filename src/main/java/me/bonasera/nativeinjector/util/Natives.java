package me.bonasera.nativeinjector.util;

import java.io.*;

/**
 * @author Andrew Bonasera
 */

public final class Natives
{
    public static File extractFromResourcePath(String dll)
    {
        try
        {
            InputStream is = Natives.class.getResourceAsStream("/" + dll + ".dll");

            if (is == null)
            {
                throw new RuntimeException("Native library " + dll + " not found");
            }

            File tmp = File.createTempFile(dll, ".dll");

            try (OutputStream out = new FileOutputStream(tmp))
            {
                byte[] buffer = new byte[1024];
                int read;
                while ((read = is.read(buffer)) != -1)
                {
                    out.write(buffer, 0, read);
                }
            }

            tmp.deleteOnExit();

            return tmp;
        } catch (IOException e)
        {
            throw new RuntimeException(e);
        }
    }
}
