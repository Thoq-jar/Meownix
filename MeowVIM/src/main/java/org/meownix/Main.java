package org.meownix;

import com.googlecode.lanterna.*;
import com.googlecode.lanterna.input.*;
import com.googlecode.lanterna.screen.*;
import com.googlecode.lanterna.terminal.*;
import org.meownix.utils.*;

import java.io.*;
import java.nio.file.*;
import java.time.*;
import java.util.*;
import java.util.stream.*;

import static java.lang.System.*;

public class Main {
    public static void main(String[] args) {
        logger.info("Starting MeowVIM...");
        logger.info("Initializing logger...");
        logger.blank();
        logger.info("Info looks like this.");
        logger.warn("Warnings looks like this.");
        logger.err("Errors looks like this.");
        logger.info("Done!");
        logger.blank();
        logger.info("========================================");
        logger.info("MeowVIM developed by Thoq & RareHyperIon");
        logger.info("Time: " + LocalDateTime.now());
        logger.info("Java Version: " + System.getProperty("java.version"));
        logger.info("Date: " + LocalDate.now());
        logger.info("========================================");
        logger.blank();
        logger.info("Initializing terminal...");
        logger.info("Creating terminal factory...");
        logger.info("Done!");
        DefaultTerminalFactory terminalFactory = new DefaultTerminalFactory();
        Screen screen = null;
        List<StringBuilder> lines = new ArrayList<>();
        lines.add(new StringBuilder());
        String filename = args.length > 0 ? args[0] : null;
        boolean commandMode = false;
        StringBuilder command = new StringBuilder();
        TerminalPosition cursorPosition = new TerminalPosition(0, 0);
        if (filename != null) {
            logger.info("Opening file: " + filename);
            try {
                List<String> fileLines = Files.readAllLines(Paths.get(filename));
                lines.clear();
                for (String fileLine : fileLines) {
                    lines.add(new StringBuilder(fileLine));
                }
            } catch (IOException e) {
                logger.warn("Attempted to open file: " + filename + " but it does not exist.");
                logger.err("Error opening file: " + e.getMessage());
            }
        }

        try {
            Terminal terminal = terminalFactory.createTerminal();
            screen = new TerminalScreen(terminal);

            screen.startScreen();
            screen.setCursorPosition(cursorPosition);

            while (true) {
                KeyStroke keyStroke = screen.pollInput();
                if (keyStroke != null) {
                    if (keyStroke.getKeyType() == KeyType.Enter) {
                        if (commandMode) {
                            logger.info("Command mode: " + command);
                            switch (command.toString()) {
                                case ":w":
                                    logger.info("Saving file...");
                                    if (filename == null) {
                                        filename = "output.txt";
                                        logger.warn("No filename specified, saving to main.c");
                                    }
                                    Files.write(Paths.get(filename), lines.stream().map(StringBuilder::toString).collect(Collectors.toList()));
                                    break;
                                case ":q":
                                    exit(0);
                                case ":qw":
                                    if (filename == null) {
                                        filename = "output.txt";
                                        logger.warn("No filename specified, saving to main.c");
                                    }
                                    Files.write(Paths.get(filename), lines.stream().map(StringBuilder::toString).collect(Collectors.toList()));
                                    logger.info("Exiting MeowVIM...");
                                    exit(0);
                                case ":q!":
                                    logger.info("Exiting MeowVIM without saving changes...");
                                    exit(0);
                            }
                            commandMode = false;
                            command.setLength(0);
                        } else {
                            logger.info("Insert Mode | Cursor Pos: " + cursorPosition);
                            int y = cursorPosition.getRow();
                            StringBuilder line = lines.get(y);
                            lines.add(y + 1, new StringBuilder(line.substring(cursorPosition.getColumn())));
                            line.setLength(cursorPosition.getColumn());
                            cursorPosition = new TerminalPosition(0, y + 1);
                        }
                    } else if (keyStroke.getKeyType() == KeyType.Character) {
                        if (commandMode) {
                            command.append(keyStroke.getCharacter());
                        } else {
                            int y = cursorPosition.getRow();
                            StringBuilder line = lines.get(y);
                            line.insert(cursorPosition.getColumn(), keyStroke.getCharacter());
                            cursorPosition = cursorPosition.withRelativeColumn(1);
                        }
                    } else if (keyStroke.getKeyType() == KeyType.Escape) {
                        commandMode = true;
                    } else if (keyStroke.getKeyType() == KeyType.Backspace) {
                        int y = cursorPosition.getRow();
                        StringBuilder line = lines.get(y);
                        if (cursorPosition.getColumn() > 0) {
                            line.deleteCharAt(cursorPosition.getColumn() - 1);
                            cursorPosition = cursorPosition.withRelativeColumn(-1);
                        } else if (line.length() == 0 && lines.size() > 1) {
                            lines.remove(y);
                            if (y > 0) {
                                cursorPosition = new TerminalPosition(lines.get(y - 1).length(), y - 1);
                            }
                        }
                    } else if (keyStroke.getKeyType() == KeyType.ArrowUp) {
                        if (cursorPosition.getRow() > 0) {
                            cursorPosition = cursorPosition.withRelativeRow(-1);
                        }
                    } else if (keyStroke.getKeyType() == KeyType.ArrowDown) {
                        if (cursorPosition.getRow() < lines.size() - 1) {
                            cursorPosition = cursorPosition.withRelativeRow(1);
                        }
                    } else if (keyStroke.getKeyType() == KeyType.ArrowLeft) {
                        if (cursorPosition.getColumn() > 0) {
                            cursorPosition = cursorPosition.withRelativeColumn(-1);
                        }
                    } else if (keyStroke.getKeyType() == KeyType.ArrowRight) {
                        if (cursorPosition.getColumn() < lines.get(cursorPosition.getRow()).length()) {
                            cursorPosition = cursorPosition.withRelativeColumn(1);
                        }
                    } else if (keyStroke.getKeyType() == KeyType.Tab) {
                        int y = cursorPosition.getRow();
                        StringBuilder line = lines.get(y);
                        line.insert(cursorPosition.getColumn(), "  ");
                        cursorPosition = cursorPosition.withRelativeColumn(2);
                    }
                    screen.clear();
                    screen.newTextGraphics().putString(0, 0, "==MeowVIM==");
                    for (int i = 0; i < lines.size(); i++) {
                        screen.newTextGraphics().putString(0, i, lines.get(i).toString());
                    }
                    if (commandMode) {
                        screen.newTextGraphics().putString(0, 0, "==MeowVIM==COMMAND==" + command);
                    } else {
                        screen.newTextGraphics().putString(0, 0, "==MeowVIM==INSERT==");
                    }
                    screen.setCursorPosition(cursorPosition);
                    screen.refresh();
                }
            }
        } catch (IOException e) {
            logger.warn("Attempted to start screen but failed.");
            logger.err("Error starting screen: " + e.getMessage());
        } finally {
            if(screen != null) {
                try {
                    screen.stopScreen();
                } catch(IOException e) {
                    logger.warn("Attempted to stop screen but failed.");
                    logger.err("Error stopping screen: " + e.getMessage());
                }
            }
        }
    }
}