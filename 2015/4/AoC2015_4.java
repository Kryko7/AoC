import java.io.BufferedReader;
import java.io.FileReader;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

public class AoC2015_4 {
    public static String parseFile(String filename) {
        try (BufferedReader br = new BufferedReader(new FileReader(filename))) {
            StringBuilder sb = new StringBuilder();
            String line;

            while((line = br.readLine()) != null) {
                sb.append(line);
            }

            return sb.toString();
        } catch (Exception e) {
            System.out.println("Error reading the file");
            return "";
        }
    }

    public static String getMD5Hash(String input) {
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            byte[] digest = md.digest(input.getBytes());

            StringBuilder sb = new StringBuilder();
            for(byte b : digest) {
                sb.append(String.format("%02x", b));
            }
            return sb.toString();
        } catch (NoSuchAlgorithmException e) {
            throw new RuntimeException("MD5 algorirhm nod found");
        }
    }

    public static int adventCoin5(String input) {
        int i = 1;
        boolean found = false;
        while(!found) {
            String s = input + i;
            String md5Hash = getMD5Hash(s);
            if(md5Hash.substring(0, 5).equals("00000")) {
                found = true;
            }
            i++;
        }
        return i-1;
    }

    public static int adventCoin6(String input) {
        int i = 1;
        boolean found = false;
        while(!found) {
            String s = input + i;
            String md5Hash = getMD5Hash(s);
            if(md5Hash.substring(0, 6).equals("000000")) {
                found = true;
            }
            i++;
        }
        return i-1;
    }
    
    public static void main(String[] args) {
        String filename = "4.txt";
        String input = parseFile(filename);
        int ans1 = adventCoin5(input);
        int ans2 = adventCoin6(input);
        System.out.println("Answer for the part 1: " + ans1);
        System.out.println("Answer for the part 2: " + ans2);
    }
}
