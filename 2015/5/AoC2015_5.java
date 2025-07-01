import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.List;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class AoC2015_5 {

    public static List<String> parseFile(String filename) {
        try (BufferedReader br = new BufferedReader(new FileReader(filename))) {
            List<String> words = new ArrayList<>();
            String line;

            while((line = br.readLine()) != null) {
                words.add(line);
            }

            return words;
        } catch (Exception e) {
            System.out.println("Error reading the file");
            return new ArrayList<>();
        }
    }

    public static int niceWords1(List<String> words) {
        int count = 0;

        Pattern doubleLetter = Pattern.compile("([a-zA-Z])\\1");
        Pattern vowels = Pattern.compile("[aeiou]");
        Pattern forbidden = Pattern.compile("ab|cd|pq|xy");

        for (String word : words) {
            Matcher m1 = doubleLetter.matcher(word);
            Matcher m2 = vowels.matcher(word);
            Matcher m3 = forbidden.matcher(word);

            int vowelsCount = 0;
            while(m2.find()) vowelsCount++;

            if (m1.find() && vowelsCount >= 3 && !m3.find()) {
                count++;
            }
        }
        return count;
    }

    public static int niceWords2(List<String> words) {
        int count = 0;

        Pattern doublePattern = Pattern.compile("(..).*\\1");
        Pattern repeat = Pattern.compile("([a-zA-Z]).\\1");

        for (String word : words) {
            Matcher m1 = repeat.matcher(word);
            Matcher m2 = doublePattern.matcher(word);

            if (m1.find() && m2.find()) {
                count++;
            }
        }
        return count;
    }

    public static void main(String[] args) {
        String filename = "5.txt";
        List<String> words = parseFile(filename);
        int ans1 = niceWords1(words);
        int ans2 = niceWords2(words);
        System.out.println("Answer for part 1: " + ans1);
        System.out.println("Answer for part 2: " + ans2);
    }
}