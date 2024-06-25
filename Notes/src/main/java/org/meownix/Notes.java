package org.meownix;

import org.meownix.Utilitys.*;

import javax.swing.*;

public class Notes {
    public static void main(String[] args) {
        SwingUtilities.invokeLater(() -> {
            LoggerUtility.info("Starting Notes...");
            LoggerUtility.blank();
            new Main().frame.setVisible(true);
        });
    }
}
