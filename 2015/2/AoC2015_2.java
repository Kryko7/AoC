import java.io.BufferedReader;
import java.io.FileReader;
import java.util.*;

public class AoC2015_2 {
    public static List<int[]> parseFile(String filename) {
        try (BufferedReader br = new BufferedReader(new FileReader(filename))) {
            ArrayList<int[]> input = new ArrayList<>();
            String line;
            while ((line = br.readLine()) != null) {
                String[] parts = line.split("x");
                int[] v = new int[3];
                for(int i=0; i<3; i++) {
                    v[i] = Integer.parseInt(parts[i].trim());
                }
                input.add(v);
            }
            return input;
        } catch (Exception e) {
            System.out.println("Error opening file");
            return new ArrayList<>();
        }
    }

    public static int wrapping_paper(List<int[]> input) {
        int total = 0;
        for(int[] v : input) {
            int wrapping = 2*v[0]*v[1] + 2*v[0]*v[2] + 2*v[1]*v[2];
            Arrays.sort(v);
            int slack = v[0]*v[1];
            total += wrapping + slack;
        }
        return total;
    }

    public static int ribbones(List<int[]> input) {
        int total = 0;
        for (int[] v : input) {
            int wrapping = v[0] * v[1] * v[2];
            Arrays.sort(v);
            int slack = 2 * v[0] + 2 * v[1];
            total += wrapping + slack;
        }
        return total;
    }

    public static void main(String[] args) {
        String filename = "2.txt";
        List<int[]> input = parseFile(filename);
        int ans1 = wrapping_paper(input);
        int ans2 = ribbones(input);
        System.out.println("Answer for part 1: " + ans1);
        System.out.println("Answer for part 2: " + ans2);
    }
}
