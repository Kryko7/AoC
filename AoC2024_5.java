import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Map;
import java.util.List;

public class AoC2024_5 {
    static Map<Integer, ArrayList<Integer>> map = new HashMap<>();
    static List<ArrayList<Integer>> grid = new ArrayList<>();

    public static void parse(String filePath) {
        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = br.readLine()) != null) {
                System.out.println("Line: " + line);
                if (line.isEmpty()) {
                    break;
                }
                String[] parts = line.split("\\|");
                int key = Integer.parseInt(parts[0]);
                int value = Integer.parseInt(parts[1]);
                if (map.containsKey(key)) {
                    map.get(key).add(value);
                } else {
                    ArrayList<Integer> list = new ArrayList<>();
                    list.add(value);
                    map.put(key, list);
                }
            }
            while ((line = br.readLine()) != null) {
                String[] paStrings = line.split(",");
                List <Integer> row = new ArrayList<>();
                for (int i = 0; i < paStrings.length; i++) {
                    row.add(Integer.parseInt(paStrings[i]));
                }
                grid.add((ArrayList<Integer>) row);
                }
            }  catch (IOException e) {
                e.printStackTrace();
        }
    }

    public static List<Integer> illegible() {
        List<Integer> result = new ArrayList<>();
        int index = 0;
        for(List<Integer> row : grid) {
            boolean good = true;
            for (int i=1; i<row.size(); i++) {
                for (int j=0; j<i; j++) {
                    if (map.getOrDefault(row.get(i), new ArrayList<>()).contains(row.get(j))) {
                        good = false;
                        break;
                    }
                    if (!good) {
                        break;
                    }
                }
            }
            if (good) {
                result.add(index);
            }
            index++;
        }
        return result;
    }

    public static List<Integer> notIllegible() {
        List<Integer> result = new ArrayList<>();
        int index = 0;
        for(List<Integer> row : grid) {
            boolean good = true;
            for (int i=1; i<row.size(); i++) {
                for (int j=0; j<i; j++) {
                    if (map.getOrDefault(row.get(i), new ArrayList<>()).contains(row.get(j))) {
                        good = false;
                        break;
                    }
                    if (!good) {
                        break;
                    }
                }
            }
            if (!good) {
                result.add(index);
            }
            index++;
        }
        return result;
    }

    public static int correctedMidCount(List<Integer> list) {
        int count = 0;
        for (int i : list) {
            List<Integer> row = grid.get(i);
            int p = row.size();
            boolean swapped;
            do {
                swapped = false;
                for (int k = 1; k < p; k++) {
                    for (int l = 0; l < k; l++) {
                        if (map.getOrDefault(row.get(k), new ArrayList<>()).contains(row.get(l))) {
                            int temp = row.get(k);
                            row.set(k, row.get(l));
                            row.set(l, temp);
                            swapped = true;
                        }
                    }
                }
            } while (swapped);
            count += row.get(p/2);
        }
        return count;
    }

    public static int midCount(List<Integer> list) {
        int count = 0;
        for (int i : list) {
            int size = grid.get(i).size();
            count += grid.get(i).get(size/2);
        }
        return count;
    }
    public static void main(String[] args) {

        parse("/home/minindu/AoC/AoC2024_5.txt");
        List<Integer> result = illegible();
        List<Integer> result2 = notIllegible();
        System.out.println("Illegible Rows: " + result);
        System.out.println("Not Illegible Rows: " + result2);
        int count = midCount(result);
        int count2 = correctedMidCount(result2);
        System.out.println("Mid Count: " + count);
        System.out.println("Corrected mid count: " + count2);
        
    }
}
