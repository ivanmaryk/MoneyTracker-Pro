# finance_python.py — финансовый учёт с категориями расходов на Python

import json
import os
import sys
from datetime import datetime
import matplotlib.pyplot as plt

class FinanceManager:
    def __init__(self):
        self.data_file = "finance_data.json"
        self.transactions = []      # список транзакций
        self.budgets = {}           # категория -> бюджет
        self.categories = set()     # все категории
        self.load_data()

    def load_data(self):
        if os.path.exists(self.data_file):
            with open(self.data_file, 'r') as f:
                data = json.load(f)
                self.transactions = data.get('transactions', [])
                self.budgets = data.get('budgets', {})
                self.categories = set(data.get('categories', []))
        else:
            self.transactions = []
            self.budgets = {}
            self.categories = set()

    def save_data(self):
        data = {
            'transactions': self.transactions,
            'budgets': self.budgets,
            'categories': list(self.categories)
        }
        with open(self.data_file, 'w') as f:
            json.dump(data, f, indent=2)

    def add_transaction(self, category, amount, comment=""):
        if category not in self.categories:
            self.categories.add(category)
        transaction = {
            "date": datetime.now().isoformat(),
            "category": category,
            "amount": round(amount, 2),
            "comment": comment
        }
        self.transactions.append(transaction)
        self.save_data()
        print(f"✅ Расход добавлен: {category} {amount:.2f} ({comment})")

    def set_budget(self, category, budget):
        self.budgets[category] = round(budget, 2)
        self.save_data()
        print(f"✅ Бюджет для {category}: {budget:.2f}")

    def get_stats(self):
        if not self.transactions:
            return None
        total = sum(t['amount'] for t in self.transactions)
        by_category = {}
        for t in self.transactions:
            cat = t['category']
            by_category[cat] = by_category.get(cat, 0) + t['amount']
        return {
            "total": total,
            "by_category": by_category,
            "count": len(self.transactions)
        }

    def budget_status(self):
        stats = self.get_stats()
        if not stats:
            return {}
        status = {}
        for cat, spent in stats['by_category'].items():
            budget = self.budgets.get(cat, None)
            if budget is not None:
                remaining = budget - spent
                percent = (spent / budget) * 100 if budget > 0 else 0
                status[cat] = {
                    'budget': budget,
                    'spent': spent,
                    'remaining': remaining,
                    'percent': percent,
                    'over': spent > budget
                }
        return status

    def recommendations(self):
        status = self.budget_status()
        if not status:
            return ["Добавьте расходы и бюджеты для получения рекомендаций."]
        over = [cat for cat, s in status.items() if s['over']]
        recs = []
        if over:
            recs.append(f"⚠️ Вы превысили бюджет по категориям: {', '.join(over)}")
        else:
            recs.append("✅ Все бюджеты в норме, отлично!")
        if len(self.transactions) < 5:
            recs.append("📝 Добавьте больше транзакций для более точного анализа.")
        return recs

    def export_csv(self, filename="finance_export.csv"):
        import csv
        with open(filename, 'w', newline='') as f:
            writer = csv.writer(f)
            writer.writerow(["Date", "Category", "Amount", "Comment"])
            for t in self.transactions:
                writer.writerow([t['date'], t['category'], t['amount'], t['comment']])
        print(f"Экспортировано в {filename}")

    def plot_pie(self):
        stats = self.get_stats()
        if not stats or not stats['by_category']:
            print("Нет данных для графика.")
            return
        labels = list(stats['by_category'].keys())
        sizes = list(stats['by_category'].values())
        plt.figure(figsize=(8, 6))
        plt.pie(sizes, labels=labels, autopct='%1.1f%%', startangle=90)
        plt.title('Распределение расходов по категориям')
        plt.axis('equal')
        plt.show()

    def interactive(self):
        print("💰 MoneyTracker Pro — Python Edition")
        print("Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>,")
        print("        stats, status, recs, pie, export, exit")
        while True:
            cmd = input("> ").strip().lower()
            if cmd == 'exit':
                break
            elif cmd.startswith('add'):
                parts = cmd.split(maxsplit=3)
                if len(parts) >= 3:
                    cat = parts[1]
                    try:
                        amount = float(parts[2])
                        comment = parts[3] if len(parts) > 3 else ""
                        self.add_transaction(cat, amount, comment)
                    except:
                        print("Ошибка ввода суммы.")
                else:
                    print("Использование: add <категория> <сумма> [комментарий]")
            elif cmd.startswith('budget'):
                parts = cmd.split()
                if len(parts) >= 3:
                    cat = parts[1]
                    try:
                        budget = float(parts[2])
                        self.set_budget(cat, budget)
                    except:
                        print("Ошибка ввода суммы.")
                else:
                    print("Использование: budget <категория> <сумма>")
            elif cmd == 'stats':
                stats = self.get_stats()
                if stats:
                    print(f"📊 Общие расходы: {stats['total']:.2f}")
                    print("По категориям:")
                    for cat, amt in sorted(stats['by_category'].items(), key=lambda x: -x[1]):
                        print(f"  {cat}: {amt:.2f}")
                    print(f"Всего транзакций: {stats['count']}")
                else:
                    print("Нет данных.")
            elif cmd == 'status':
                status = self.budget_status()
                if not status:
                    print("Нет бюджетов или расходов.")
                else:
                    print("📊 Бюджетный статус:")
                    for cat, s in status.items():
                        sign = "⚠️" if s['over'] else "✅"
                        print(f"  {sign} {cat}: потрачено {s['spent']:.2f}/{s['budget']:.2f} "
                              f"({s['percent']:.1f}%), осталось {s['remaining']:.2f}")
            elif cmd == 'recs':
                recs = self.recommendations()
                print("💡 Рекомендации:")
                for r in recs:
                    print(f"  {r}")
            elif cmd == 'pie':
                self.plot_pie()
            elif cmd == 'export':
                self.export_csv()
            else:
                print("Неизвестная команда.")

if __name__ == "__main__":
    fm = FinanceManager()
    fm.interactive()
