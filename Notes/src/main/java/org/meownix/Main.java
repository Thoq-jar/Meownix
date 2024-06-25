package org.meownix;

import javax.swing.*;
import java.awt.*;
import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

import org.meownix.Utilitys.*;

public class Main {
    public String NAME = "Notes";
    final JFrame frame;
    private final JTextArea textArea;
    private final DefaultListModel<String> listModel;
    private final JList<String> list;
    private final String NOTES_FILE = "notes.txt";

    public Main() {
        listModel = new DefaultListModel<>();
        loadNotes();
        frame = new JFrame(NAME);
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(1000, 500);

        try {
            for (UIManager.LookAndFeelInfo info : UIManager.getInstalledLookAndFeels()) {
                if ("Nimbus".equals(info.getName())) {
                    LoggerUtility.info("Loading Nimbus theme...");
                    UIManager.setLookAndFeel(info.getClassName());
                    UIManager.put("control", new Color(30, 30, 30));
                    UIManager.put("info", new Color(30, 30, 30));
                    UIManager.put("nimbusBase", new Color(18, 30, 49));
                    UIManager.put("nimbusAlertYellow", new Color(248, 187, 0));
                    UIManager.put("nimbusDisabledText", new Color(128, 128, 128));
                    UIManager.put("nimbusFocus", new Color(115, 164, 209));
                    UIManager.put("nimbusGreen", new Color(176, 179, 50));
                    UIManager.put("nimbusInfoBlue", new Color(66, 139, 221));
                    UIManager.put("nimbusLightBackground", new Color(30, 30, 30));
                    UIManager.put("nimbusOrange", new Color(191, 98, 4));
                    UIManager.put("nimbusRed", new Color(169, 46, 34));
                    UIManager.put("nimbusSelectedText", new Color(255, 255, 255));
                    UIManager.put("nimbusSelectionBackground", new Color(104, 93, 156));
                    UIManager.put("text", new Color(255, 255, 255));
                    break;
                }
            }
        } catch (Exception e) {
            LoggerUtility.warn("Failed to set dark theme! Continuing with default theme...");
            LoggerUtility.err(e.getMessage());
        }

        list = new JList<>(listModel);
        list.setSelectionMode(ListSelectionModel.SINGLE_SELECTION);
        list.setLayoutOrientation(JList.VERTICAL);
        list.setVisibleRowCount(-1);
        JScrollPane listScroller = new JScrollPane(list);
        listScroller.setPreferredSize(new Dimension(250, 80));

        textArea = new JTextArea();
        JScrollPane textAreaScroller = new JScrollPane(textArea);

        JPanel buttonPanel = getjPanel();

        frame.getContentPane().add(listScroller, BorderLayout.WEST);
        frame.getContentPane().add(textAreaScroller, BorderLayout.CENTER);
        frame.getContentPane().add(buttonPanel, BorderLayout.SOUTH);
    }

    private void saveNotes() {
        try (BufferedWriter writer = Files.newBufferedWriter(Paths.get(NOTES_FILE))) {
            for (int i = 0; i < listModel.getSize(); i++) {
                writer.write(listModel.getElementAt(i));
                writer.newLine();
            }
        } catch (IOException e) {
            LoggerUtility.warn("Failed to save notes!");
            LoggerUtility.err(e.getMessage());
        }
    }

    private void loadNotes() {
        try (BufferedReader reader = Files.newBufferedReader(Paths.get(NOTES_FILE))) {
            String line;
            while ((line = reader.readLine()) != null) {
                assert listModel != null;
                listModel.addElement(line);
            }
        } catch (IOException e) {
            LoggerUtility.warn("Failed to load notes!");
            LoggerUtility.err(e.getMessage());
        }
    }

    private JPanel getjPanel() {
        JButton addButton = new JButton("Add Note");
        addButton.addActionListener(e -> {
            listModel.addElement(textArea.getText());
            textArea.setText("");
            saveNotes();
        });

        JButton editButton = new JButton("Edit Note");
        editButton.addActionListener(e -> {
            int selectedIndex = list.getSelectedIndex();
            if (selectedIndex != -1) {
                listModel.set(selectedIndex, textArea.getText());
                saveNotes();
            }
        });

        JButton deleteButton = new JButton("Delete Note");
        deleteButton.addActionListener(e -> {
            int selectedIndex = list.getSelectedIndex();
            if (selectedIndex != -1) {
                listModel.remove(selectedIndex);
                saveNotes();
            }
        });

        JPanel buttonPanel = new JPanel();
        buttonPanel.add(addButton);
        buttonPanel.add(editButton);
        buttonPanel.add(deleteButton);
        return buttonPanel;
    }
}
