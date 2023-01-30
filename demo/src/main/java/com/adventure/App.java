package com.adventure;
import java.awt.BorderLayout;
import java.awt.Container;
import java.awt.event.KeyEvent;
import java.awt.event.KeyListener;
import java.util.ArrayList;

import javax.swing.JFrame;
import javax.swing.JTextField;

public class App {

    private String getUserInputUntilEnterIsPressed(){
        
        String input = "";


        return "asd";
    }

    private boolean listen = false;

    public static void main(String args[]) {

        // keyboard listener
        JFrame frame = new JFrame("Game listener");
        Container contentPane = frame.getContentPane();
        KeyListener listener = new KeyListener() {
            @Override
            public void keyPressed(KeyEvent event) {
                // System.out.println( event.getKeyChar() );
            }
            @Override
            public void keyReleased(KeyEvent event) {
                // System.out.println( event.getKeyChar() );
            }

            @Override
            public void keyTyped(KeyEvent event) {
              System.out.println( event.getKeyChar());
            }
        
        };
        // idk text field in the shit
        JTextField textField = new JTextField(); // some textfield bs
        textField.addKeyListener(listener); // adds key listener
        contentPane.add(textField, BorderLayout.NORTH); // idk
        frame.pack(); // idk wtf this does
        frame.setVisible(true); // maybe sets it visible? idk

        
    }
}