package me.bonasera.nativeinjector;

import me.bonasera.nativeinjector.util.Natives;

import java.io.*;
import java.util.Scanner;

/**
 * @author Andrew Bonasera
 */

public final class Main
{
    /*
     * Loads the native injection library.
     */
    static
    {
        System.out.println("\nLoading native injection library...");

        File injector = Natives.extractFromResourcePath("injector");
        System.load(injector.getAbsolutePath());
    }

    /**
     * Lists the open virtual machines, then awaits an input from the user.
     *
     * The requested PID is then passed to the native portion of the code.
     */
    public static void main(String[] args)
    {
        System.out.print("\nOpen Virtual Machines:\n");

        try
        {
            Process jps = Runtime.getRuntime().exec("jps");
            BufferedReader reader = new BufferedReader(new InputStreamReader(jps.getInputStream()));

            String line;
            while ((line = reader.readLine()) != null)
            {
                String[] parts = line.split(" ");
                if (parts.length >= 2)
                {
                    System.out.printf("%s, PID=%s\n", parts[1], parts[0]);
                }
            }

            jps.waitFor();
            reader.close();
        } catch (IOException | InterruptedException e)
        {
            throw new RuntimeException("Failed to collect open virtual machines", e);
        }

        System.out.print("\nEnter the PID of the program you wish to inject: ");

        int pid;
        try (Scanner scanner = new Scanner(System.in))
        {

            pid = Integer.parseInt(scanner.nextLine());
        } catch (NumberFormatException e)
        {
            throw new RuntimeException("PID must include only numbers", e);
        }

        File payload = Natives.extractFromResourcePath("payload");
        inject(pid, payload.getAbsolutePath());
    }

    private static native void inject(int pid,
                                      String payloadPath);
}
