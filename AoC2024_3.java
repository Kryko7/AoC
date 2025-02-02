import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class AoC2024_3 {
    public static String parse(String filePath) {
        String content = "";
        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = br.readLine()) != null) {
                content += line;
            }
        } catch (IOException e) {
            e.printStackTrace();
        }
        return content;
    }

    public static int countPartOne(String input) {
        String regex = "mul\\((\\d+),\\s*(\\d+)\\)";
        Pattern pattern = Pattern.compile(regex);
        Matcher matcher = pattern.matcher(input);

        int result = 0;
        while (matcher.find()) {
            String num1 = matcher.group(1);
            String num2 = matcher.group(2);
            result += Integer.parseInt(num1) * Integer.parseInt(num2);
        }
        return result;
    }

    public static int countPartTwo(String input) {
        String regex = "(mul\\((\\d+),\\s*(\\d+)\\)|do\\(\\)|don't\\(\\))";
        Pattern pattern = Pattern.compile(regex);
        Matcher matcher = pattern.matcher(input);

        int result = 0;
        boolean doMul = true;
        while(matcher.find()) {
            String group = matcher.group();
            if (group.equals("do()")) {
                doMul = true;
                continue;
            } else if (group.equals("don't()")) {
                doMul = false;
                continue;
            } else if (doMul) {
                String num1 = matcher.group(2);
                String num2 = matcher.group(3);
                result += Integer.parseInt(num1) * Integer.parseInt(num2);
            }
        }
        return result;
    }
    public static void main(String[] args) {
        // String input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        String input = parse("/home/minindu/AoC/AoC2024_3.txt");
        int result = countPartOne(input);
        int result2 = countPartTwo(input);
        System.out.println("Result: " + result);
        System.out.println("Result2: " + result2);
    }
}