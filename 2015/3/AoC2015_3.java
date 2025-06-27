import java.io.BufferedReader;
import java.io.FileReader;
import java.util.HashSet;
import java.util.Objects;
import java.util.Set;


class Location {
    int x, y;

    public Location(int x, int y) {
        this.x = x;
        this.y = y;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Location)) return false;
        Location loc = (Location) o;
        return x == loc.x && y == loc.y;
    }

    @Override
    public int hashCode() {
        return Objects.hash(x, y);
    }
}

public class AoC2015_3 {
    public static char[] parseFile(String filename) {
        try (BufferedReader br = new BufferedReader(new FileReader(filename))) {
            StringBuilder sb = new StringBuilder();
            String line;

            while ((line = br.readLine()) != null) {
                sb.append(line);
            }

            return sb.toString().toCharArray();

        } catch (Exception e) {
            System.out.println("Error occurred while reading the file: " + e.getMessage());
            return new char[0];
        }
    }

    public static int uniqueHouses(char[] input) {
        int x = 0;
        int y = 0;
        Set<Location> locations = new HashSet<>();
        Location initLocation = new Location(x, y);
        locations.add(initLocation);
        for(char c : input) {
            if(c == '>') {
                x++;
            } else if (c == '<') {
                x--;
            } else if (c == '^') {
                y++;
            } else {
                y--;
            }
            locations.add(new Location(x, y));
        }
        return locations.size();
    }

    public static int uniqueHouses2(char[] input) {
        int x = 0;
        int y = 0;
        int a = 0;
        int b = 0;
        Set<Location> locations = new HashSet<>();
        Location initLocation = new Location(x, y);
        locations.add(initLocation);
        for (int i = 0; i < input.length; i++) {
            char c = input[i];
    
            if (i % 2 == 0) { 
                if (c == '>') x++;
                else if (c == '<') x--;
                else if (c == '^') y++;
                else if (c == 'v') y--;
                locations.add(new Location(x, y));
            } else { 
                if (c == '>') a++;
                else if (c == '<') a--;
                else if (c == '^') b++;
                else if (c == 'v') b--;
                locations.add(new Location(a, b));
            }
        }
        return locations.size();
    }

    public static void main(String[] args) {
        String filename = "3.txt";
        char[] input = parseFile(filename);
        int ans1 = uniqueHouses(input);
        int ans2 = uniqueHouses2(input);
        System.out.println("Answer for part1: " + ans1);
        System.out.println("Answer for part2: " + ans2);
    }
}
