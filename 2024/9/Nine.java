package AoC2024_9;

import java.io.BufferedReader;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.List;

public class Nine {
    private String parsing(String filename) {
        try(BufferedReader br = new BufferedReader(new FileReader(filename))) {
            String line = br.readLine();
            return line;
        } catch (Exception e) {
            e.printStackTrace();
        }
        return null;
    }

    private List<Integer> mapping(String line) {
        List<Integer> list = new ArrayList<>();
        int index = 0;
        for (int i = 0; i < line.length(); i++) {
            int num = line.charAt(i) - '0';
            if(i % 2 == 0) {
                for (int j = 0; j < num; j++) {
                    list.add(index);
                }
                index++;
            } else {
                for (int j = 0; j < num; j++) {
                    list.add(-1);
                }
            }
        }
        return list;
    }

    private int[] swapping(List<Integer> list) {
        int[] arr = list.stream().mapToInt(i -> i).toArray();
        int left = 0;
        int right = arr.length - 1;
        while(left < right) {
            while(left < right && arr[left] != -1) {
                left++;
            }
            while(left < right && arr[right] == -1) {
                right--;
            }
            if(left < right) {
                arr[left] = arr[right];
                arr[right] = -1;
                left++;
                right--;
            }
        }
        return arr;
    }

    private long calculateCecksum(int[] arr) {
        long checksum = 0;
        int index = 0;
        while(index < arr.length && arr[index] != -1) {
            checksum += arr[index] * index;
            index++;
        }
        return checksum;
    }

    public static void main(String[] args) {
        Nine nine = new Nine();
        String line = nine.parsing("/home/minindu/AoC/AoC2024_9/AoC2024_9.txt");
        List<Integer> list = nine.mapping(line);
        int[] arr = nine.swapping(list);
        long checksum = nine.calculateCecksum(arr);
        System.out.println("Checksum: " + checksum);
    }
}
