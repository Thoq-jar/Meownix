package org.meownix.Utilitys;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;

public class LoggerUtility {
    private static final String ANSI_RESET = "\u001B[0m";
    private static final String ANSI_RED = "\u001B[31m";
    private static final String ANSI_YELLOW = "\u001B[33m";
    private static final String ANSI_GREEN = "\u001B[32m";

    private static String getCurrentTime() {
        LocalDateTime now = LocalDateTime.now();
        DateTimeFormatter formatter = DateTimeFormatter.ofPattern("HH:mm:ss");
        return now.format(formatter);
    }

    public static void err(String message) {
        System.out.println(ANSI_RED + getCurrentTime() + ": " + "[THREAD/ERROR] " + message + ANSI_RESET);
    }

    public static void info(String message) {
        System.out.println(ANSI_GREEN + getCurrentTime() + ": " + "[THREAD/INFO] " + message + ANSI_RESET);
    }

    public static void warn(String message) {
        System.out.println(ANSI_YELLOW + getCurrentTime() + ": " + "[THREAD/WARN] " + message + ANSI_RESET);
    }

    public static void blank() {
        System.out.println(" ");
    }
}