package com.adventure;
import java.io.BufferedReader;
import java.io.InputStreamReader;

public class App 
{
    public static void main( String[] args )
    {
        try {
            // Enter data using BufferReader
        BufferedReader reader = new BufferedReader( new InputStreamReader(System.in) );

        System.out.println("Enter username:");

        String player = reader.readLine();
        System.out.println( player );
        
        } catch (Exception e) {
            System.out.println(e);   
        }
        
    }
}
