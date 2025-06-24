import java.io.BufferedReader;
import java.io.FileReader;

public class AoC2015_1 {

    public static String parseFile(String filename) {
        try (BufferedReader br = new BufferedReader(new FileReader(filename))) {
            String text = "";
            String line;
            while ((line = br.readLine()) != null) {
                text += line;
            }
            return text;
        } catch (Exception e) {
            System.out.println("Error opening file");
            return "";
        }
        
    }

    public static int floorCounter(String text) {
        int floor = 0;
        for(char c : text.toCharArray()) {
            if(c == '(') {
                floor++;
            } else {
                floor--;
            }
        }
        return floor;
    }

    public static int firstChange(String text) {
        int floor = 0;
        char[] charArray = text.toCharArray();
        int n = charArray.length;
        for(int i=0; i<n; i++) {
            if(charArray[i] == '(') {
                floor++;
            } else {
                floor--;
                if(floor == -1) {
                    return i + 1;
                }
            }
        }
        return -1;
    }
    
    public static void main(String[] args) {
        String filename = "1.txt";
        String text = parseFile(filename);
        int ans1 = floorCounter(text);
        int ans2 = firstChange(text);
        System.out.println("Answer for part1: " + ans1);
        System.out.println("Answer for part2: " + ans2);
    }
}