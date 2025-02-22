import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.concurrent.Callable;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.Future;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.TimeoutException;

public class AoC2024_6 {
    public static List<List<String>> grid = new ArrayList<>();
    public static int[][] arrGrid;
    public static int[][] loopGrid;
    public static int start_x;
    public static int start_y;

    public static void parse(String filePath) {
        try (BufferedReader br = new BufferedReader(new FileReader(filePath))) {
            String line;
            while ((line = br.readLine()) != null) {
                String[] parts = line.split("");
                List<String> row = new ArrayList<>(Arrays.asList(parts));
                grid.add(row);
            }

            int m = grid.size();
            int n = grid.get(0).size();
            arrGrid = new int[m][n];
            loopGrid = new int[m][n];
            for (int i = 0; i < m; i++) {
                List<String> row = grid.get(i);
                for (int j = 0; j < n; j++) {
                    if(row.get(j).equals("#")) {
                        arrGrid[i][j] = 1;
                        loopGrid[i][j] = 1;
                    } else if (row.get(j).equals("^")) {
                        arrGrid[i][j] = 0;
                        loopGrid[i][j] = 0;
                        start_x = i;
                        start_y = j;
                    } else {
                        arrGrid[i][j] = 0;
                        loopGrid[i][j] = 0;
                    }
                }
            }
        }  catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static int countUniquePartOne(int[][] newGrid) {
        int x = start_x;
        int y = start_y;
        int prev_x = x;
        int prev_y = y;
        int direction = 0;
        while (true) {
            if (x < 0 || x >= newGrid.length || y < 0 || y >= newGrid[0].length) {
                break;
            }
            if (newGrid[x][y] == 1) {
                x = prev_x;
                y = prev_y;
                direction = (direction + 1) % 4;
            } else {
                newGrid[x][y] = 2;
                prev_x = x;
                prev_y = y;

                if (direction == 0) {
                    x--;
                } else if (direction == 1) {
                    y++;
                } else if (direction == 2) {
                    x++;
                } else {
                    y--;
                }
            }
        }
        int count = 0;
        for (int[] row : newGrid) {
            for (int cell : row) {
                if (cell == 2) {
                    count++;
                }
            }
        }
        return count;
    }


    public static void helper(int[][] newGrid) {
        int x = start_x;
        int y = start_y;
        int prev_x = x;
        int prev_y = y;
        int direction = 0;
        int k = 0;
        while (true) {
            if (x < 0 || x >= newGrid.length || y < 0 || y >= newGrid[0].length) {
                break;
            }
            if (newGrid[x][y] == 1) {
                x = prev_x;
                y = prev_y;
                direction = (direction + 1) % 4;
            
            } else if (newGrid[x][y] == 0) {
                break;
                
            } else {
                newGrid[x][y] = 3;
                prev_x = x;
                prev_y = y;

                if (direction == 0) {
                    x--;
                } else if (direction == 1) {
                    y++;
                } else if (direction == 2) {
                    x++;
                } else {
                    y--;
                }
            }
        }
    }


    public static int processLocation(int x, int y, ExecutorService executor) {
        int count = 0;
    
        int[][] newGrid = arrGrid.clone();
        newGrid[x][y] = 1;
    
        Callable<Void> task = () -> {
            helper(newGrid);
            return null;
        };
    
        Future<Void> future = executor.submit(task);
    
        try {
            future.get(2, TimeUnit.SECONDS);
            System.out.println("Task completed for location: X = " + x + " Y = " + y);
        } catch (TimeoutException e) {
            System.out.println("Task timed out for location: X = " + x + " Y = " + y);
            count++;
            future.cancel(true);
        } catch (Exception e) {
            e.printStackTrace();
        }
    
        return count;
    }

    public static int countUniquePartTwo(int unique_locations) {
        int m = arrGrid.length;
        int n = arrGrid[0].length;
        int[] Xs = new int[unique_locations];
        int[] Ys = new int[unique_locations];
        int a = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (arrGrid[i][j] == 2) {
                    Xs[a] = i;
                    Ys[a] = j;
                    a++;
                }
            }
        }

        int count = 0;
        for (int i = 0; i < unique_locations; i++) {
            int[][] newGrid = new int[arrGrid.length][];
            for (int j = 0; j < arrGrid.length; j++) {
                newGrid[j] = arrGrid[j].clone();
            }
            if (Xs[i] == start_x && Ys[i] == start_y) continue;
            newGrid[Xs[i]][Ys[i]] = 1;
            System.out.println(Xs[i] + " " + Ys[i]);
        
            Thread taskThread = new Thread(() -> {
                try {
                    helper(newGrid);
                } catch (Exception e) {
                    e.printStackTrace();
                }
            });
        
            taskThread.start();
        
            try {
                taskThread.join(2000); 
                if (taskThread.isAlive()) {
                    taskThread.interrupt(); 
                    System.out.println("Task timed out for index: " + i);
                    count++;
                } else {
                    System.out.println("Task completed for index: " + i);
                }
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
        System.out.println("Count of timeouts: " + count);
        return count;
    }

    public static void main(String[] args) {
        parse("/home/minindu/AoC/AoC2024_6.txt");
        int result = countUniquePartOne(arrGrid);
        int result2 = countUniquePartTwo(result);
        System.out.println("Result Part One: " + result);
        System.out.println("Result Part Two: " + result2);
    }
}
