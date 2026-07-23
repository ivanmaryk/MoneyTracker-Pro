// finance_java.java — финансовый учёт с категориями расходов на Java

import java.io.*;
import java.nio.file.*;
import java.time.*;
import java.time.format.*;
import java.util.*;
import java.util.stream.*;

public class FinanceManager {
    private static class Transaction {
        String date;
        String category;
        double amount;
        String comment;
    }

    private List<Transaction> transactions = new ArrayList<>();
    private Map<String, Double> budgets = new HashMap<>();
    private Set<String> categories = new HashSet<>();
    private String dataFile = "finance_data.json";

    public FinanceManager() {
        loadData();
    }

    private void loadData() {
        try {
            String content = new String(Files.readAllBytes(Paths.get(dataFile)));
            // упрощённый парсинг (пропускаем для демонстрации)
        } catch (IOException e) {}
    }

    private void saveData() {
        try (PrintWriter pw = new PrintWriter(dataFile)) {
            pw.println("{");
            pw.println("\"transactions\": [");
            for (int i = 0; i < transactions.size(); i++) {
                Transaction t = transactions.get(i);
                pw.printf("{\"date\":\"%s\",\"category\":\"%s\",\"amount\":%.2f,\"comment\":\"%s\"}%s\n",
                    t.date, t.category, t.amount, t.comment, i < transactions.size()-1 ? "," : "");
            }
            pw.println("],");
            pw.println("\"budgets\": {");
            int bSize = budgets.size();
            int bIdx = 0;
            for (Map.Entry<String, Double> e : budgets.entrySet()) {
                pw.printf("\"%s\": %.2f%s\n", e.getKey(), e.getValue(), ++bIdx < bSize ? "," : "");
            }
            pw.println("}");
            pw.println("}");
        } catch (IOException e) {}
    }

    private String now() {
        return LocalDateTime.now().format(DateTimeFormatter.ISO_LOCAL_DATE_TIME);
    }

    public void addTransaction(String category, double amount, String comment) {
        if (!categories.contains(category)) categories.add(category);
        Transaction t = new Transaction();
        t.date = now();
        t.category = category;
        t.amount = amount;
        t.comment = comment;
        transactions.add(t);
        saveData();
        System.out.printf("✅ Расход добавлен: %s %.2f (%s)\n", category, amount, comment);
    }

    public void setBudget(String category, double budget) {
        budgets.put(category, budget);
        saveData();
        System.out.printf("✅ Бюджет для %s: %.2f\n", category, budget);
    }

    public void stats() {
        if (transactions.isEmpty()) { System.out.println("Нет данных."); return; }
        double total = transactions.stream().mapToDouble(t -> t.amount).sum();
        Map<String, Double> byCategory = transactions.stream()
            .collect(Collectors.groupingBy(t -> t.category, Collectors.summingDouble(t -> t.amount)));
        System.out.printf("📊 Общие расходы: %.2f\n", total);
        System.out.println("По категориям:");
        byCategory.entrySet().stream()
            .sorted((a,b) -> Double.compare(b.getValue(), a.getValue()))
            .forEach(e -> System.out.printf("  %s: %.2f\n", e.getKey(), e.getValue()));
        System.out.println("Всего транзакций: " + transactions.size());
    }

    public void budgetStatus() {
        if (budgets.isEmpty() || transactions.isEmpty()) {
            System.out.println("Нет бюджетов или расходов.");
            return;
        }
        Map<String, Double> spent = transactions.stream()
            .collect(Collectors.groupingBy(t -> t.category, Collectors.summingDouble(t -> t.amount)));
        System.out.println("📊 Бюджетный статус:");
        for (Map.Entry<String, Double> e : budgets.entrySet()) {
            String cat = e.getKey();
            double budget = e.getValue();
            double sp = spent.getOrDefault(cat, 0.0);
            double rem = budget - sp;
            double pct = budget > 0 ? (sp / budget) * 100 : 0;
            String sign = sp > budget ? "⚠️" : "✅";
            System.out.printf("  %s %s: потрачено %.2f/%.2f (%.1f%%), осталось %.2f\n",
                sign, cat, sp, budget, pct, rem);
        }
    }

    public void recommendations() {
        if (transactions.isEmpty() || budgets.isEmpty()) {
            System.out.println("Добавьте расходы и бюджеты для получения рекомендаций.");
            return;
        }
        Map<String, Double> spent = transactions.stream()
            .collect(Collectors.groupingBy(t -> t.category, Collectors.summingDouble(t -> t.amount)));
        boolean over = budgets.entrySet().stream()
            .anyMatch(e -> spent.getOrDefault(e.getKey(), 0.0) > e.getValue());
        System.out.println("💡 Рекомендации:");
        if (over) System.out.println("  ⚠️ Вы превысили бюджет по некоторым категориям.");
        else System.out.println("  ✅ Все бюджеты в норме, отлично!");
        if (transactions.size() < 5) System.out.println("  📝 Добавьте больше транзакций для точного анализа.");
    }

    public void exportCSV(String filename) {
        try (PrintWriter pw = new PrintWriter(filename)) {
            pw.println("Date,Category,Amount,Comment");
            for (Transaction t : transactions) {
                pw.printf("%s,%s,%.2f,%s\n", t.date, t.category, t.amount, t.comment);
            }
            System.out.println("Экспортировано в " + filename);
        } catch (IOException e) {}
    }

    public void interactive() {
        System.out.println("💰 MoneyTracker Pro — Java Edition");
        System.out.println("Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>,");
        System.out.println("        stats, status, recs, export, exit");
        Scanner sc = new Scanner(System.in);
        while (true) {
            System.out.print("> ");
            String cmd = sc.nextLine().trim().toLowerCase();
            if (cmd.equals("exit")) break;
            else if (cmd.startsWith("add")) {
                String[] parts = cmd.split(" ", 4);
                if (parts.length >= 3) {
                    try {
                        double amount = Double.parseDouble(parts[2]);
                        String comment = parts.length > 3 ? parts[3] : "";
                        addTransaction(parts[1], amount, comment);
                    } catch (NumberFormatException e) {
                        System.out.println("Ошибка ввода суммы.");
                    }
                } else {
                    System.out.println("Использование: add <категория> <сумма> [комментарий]");
                }
            } else if (cmd.startsWith("budget")) {
                String[] parts = cmd.split(" ");
                if (parts.length >= 3) {
                    try {
                        double budget = Double.parseDouble(parts[2]);
                        setBudget(parts[1], budget);
                    } catch (NumberFormatException e) {
                        System.out.println("Ошибка ввода суммы.");
                    }
                } else {
                    System.out.println("Использование: budget <категория> <сумма>");
                }
            } else if (cmd.equals("stats")) {
                stats();
            } else if (cmd.equals("status")) {
                budgetStatus();
            } else if (cmd.equals("recs")) {
                recommendations();
            } else if (cmd.equals("export")) {
                exportCSV("finance_export.csv");
            } else {
                System.out.println("Неизвестная команда.");
            }
        }
        sc.close();
    }

    public static void main(String[] args) {
        new FinanceManager().interactive();
    }
}
