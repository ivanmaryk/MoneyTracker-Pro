// finance_cs.cs — финансовый учёт с категориями расходов на C#

using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;

class FinanceManager
{
    private class Transaction
    {
        public string Date { get; set; }
        public string Category { get; set; }
        public double Amount { get; set; }
        public string Comment { get; set; }
    }

    private List<Transaction> transactions = new List<Transaction>();
    private Dictionary<string, double> budgets = new Dictionary<string, double>();
    private HashSet<string> categories = new HashSet<string>();
    private string dataFile = "finance_data.json";

    public FinanceManager() => LoadData();

    private void LoadData()
    {
        if (File.Exists(dataFile))
        {
            try
            {
                string json = File.ReadAllText(dataFile);
                // упрощённо, для демонстрации
            }
            catch { }
        }
    }

    private void SaveData()
    {
        var data = new
        {
            transactions = transactions,
            budgets = budgets,
            categories = categories.ToList()
        };
        string json = JsonSerializer.Serialize(data, new JsonSerializerOptions { WriteIndented = true });
        File.WriteAllText(dataFile, json);
    }

    private string Now() => DateTime.Now.ToString("o");

    public void AddTransaction(string category, double amount, string comment = "")
    {
        if (!categories.Contains(category)) categories.Add(category);
        var t = new Transaction { Date = Now(), Category = category, Amount = amount, Comment = comment };
        transactions.Add(t);
        SaveData();
        Console.WriteLine($"✅ Расход добавлен: {category} {amount:F2} ({comment})");
    }

    public void SetBudget(string category, double budget)
    {
        budgets[category] = budget;
        SaveData();
        Console.WriteLine($"✅ Бюджет для {category}: {budget:F2}");
    }

    public void Stats()
    {
        if (!transactions.Any()) { Console.WriteLine("Нет данных."); return; }
        double total = transactions.Sum(t => t.Amount);
        var byCategory = transactions.GroupBy(t => t.Category)
            .ToDictionary(g => g.Key, g => g.Sum(t => t.Amount));
        Console.WriteLine($"📊 Общие расходы: {total:F2}");
        Console.WriteLine("По категориям:");
        foreach (var kv in byCategory.OrderByDescending(kv => kv.Value))
            Console.WriteLine($"  {kv.Key}: {kv.Value:F2}");
        Console.WriteLine($"Всего транзакций: {transactions.Count}");
    }

    public void BudgetStatus()
    {
        if (!budgets.Any() || !transactions.Any())
        {
            Console.WriteLine("Нет бюджетов или расходов.");
            return;
        }
        var spent = transactions.GroupBy(t => t.Category)
            .ToDictionary(g => g.Key, g => g.Sum(t => t.Amount));
        Console.WriteLine("📊 Бюджетный статус:");
        foreach (var kv in budgets)
        {
            string cat = kv.Key;
            double budget = kv.Value;
            double sp = spent.GetValueOrDefault(cat, 0);
            double rem = budget - sp;
            double pct = budget > 0 ? (sp / budget) * 100 : 0;
            string sign = sp > budget ? "⚠️" : "✅";
            Console.WriteLine($"  {sign} {cat}: потрачено {sp:F2}/{budget:F2} ({pct:F1}%), осталось {rem:F2}");
        }
    }

    public void Recommendations()
    {
        if (!transactions.Any() || !budgets.Any())
        {
            Console.WriteLine("Добавьте расходы и бюджеты для получения рекомендаций.");
            return;
        }
        var spent = transactions.GroupBy(t => t.Category)
            .ToDictionary(g => g.Key, g => g.Sum(t => t.Amount));
        bool over = budgets.Any(kv => spent.GetValueOrDefault(kv.Key, 0) > kv.Value);
        Console.WriteLine("💡 Рекомендации:");
        if (over) Console.WriteLine("  ⚠️ Вы превысили бюджет по некоторым категориям.");
        else Console.WriteLine("  ✅ Все бюджеты в норме, отлично!");
        if (transactions.Count < 5) Console.WriteLine("  📝 Добавьте больше транзакций для точного анализа.");
    }

    public void ExportCSV(string filename = "finance_export.csv")
    {
        using var sw = new StreamWriter(filename);
        sw.WriteLine("Date,Category,Amount,Comment");
        foreach (var t in transactions)
            sw.WriteLine($"{t.Date},{t.Category},{t.Amount:F2},{t.Comment}");
        Console.WriteLine($"Экспортировано в {filename}");
    }

    public void Interactive()
    {
        Console.WriteLine("💰 MoneyTracker Pro — C# Edition");
        Console.WriteLine("Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>,");
        Console.WriteLine("        stats, status, recs, export, exit");
        while (true)
        {
            Console.Write("> ");
            string cmd = Console.ReadLine()?.Trim().ToLower() ?? "";
            if (cmd == "exit") break;
            else if (cmd.StartsWith("add"))
            {
                var parts = cmd.Split(' ', 4);
                if (parts.Length >= 3 && double.TryParse(parts[2], out double amount))
                {
                    string comment = parts.Length > 3 ? parts[3] : "";
                    AddTransaction(parts[1], amount, comment);
                }
                else Console.WriteLine("Использование: add <категория> <сумма> [комментарий]");
            }
            else if (cmd.StartsWith("budget"))
            {
                var parts = cmd.Split(' ');
                if (parts.Length >= 3 && double.TryParse(parts[2], out double budget))
                    SetBudget(parts[1], budget);
                else Console.WriteLine("Использование: budget <категория> <сумма>");
            }
            else if (cmd == "stats") Stats();
            else if (cmd == "status") BudgetStatus();
            else if (cmd == "recs") Recommendations();
            else if (cmd == "export") ExportCSV();
            else Console.WriteLine("Неизвестная команда.");
        }
    }

    public static void Main() => new FinanceManager().Interactive();
}
