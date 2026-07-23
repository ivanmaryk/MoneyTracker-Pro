// finance_js.js — финансовый учёт с категориями расходов на JavaScript (Node.js)

const fs = require('fs');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    prompt: '> '
});

class FinanceManager {
    constructor() {
        this.transactions = [];
        this.budgets = {};
        this.categories = new Set();
        this.dataFile = 'finance_data.json';
        this.loadData();
    }

    loadData() {
        try {
            const data = fs.readFileSync(this.dataFile, 'utf8');
            const parsed = JSON.parse(data);
            this.transactions = parsed.transactions || [];
            this.budgets = parsed.budgets || {};
            this.categories = new Set(parsed.categories || []);
        } catch (e) {
            this.transactions = [];
            this.budgets = {};
            this.categories = new Set();
        }
    }

    saveData() {
        const data = {
            transactions: this.transactions,
            budgets: this.budgets,
            categories: Array.from(this.categories)
        };
        fs.writeFileSync(this.dataFile, JSON.stringify(data, null, 2));
    }

    addTransaction(category, amount, comment = '') {
        if (!this.categories.has(category)) this.categories.add(category);
        const t = {
            date: new Date().toISOString(),
            category,
            amount: Math.round(amount * 100) / 100,
            comment
        };
        this.transactions.push(t);
        this.saveData();
        console.log(`✅ Расход добавлен: ${category} ${amount.toFixed(2)} (${comment})`);
    }

    setBudget(category, budget) {
        this.budgets[category] = Math.round(budget * 100) / 100;
        this.saveData();
        console.log(`✅ Бюджет для ${category}: ${budget.toFixed(2)}`);
    }

    stats() {
        if (this.transactions.length === 0) {
            console.log('Нет данных.');
            return;
        }
        const total = this.transactions.reduce((s, t) => s + t.amount, 0);
        const byCategory = {};
        for (const t of this.transactions) {
            byCategory[t.category] = (byCategory[t.category] || 0) + t.amount;
        }
        console.log(`📊 Общие расходы: ${total.toFixed(2)}`);
        console.log('По категориям:');
        const sorted = Object.entries(byCategory).sort((a, b) => b[1] - a[1]);
        for (const [cat, amt] of sorted) {
            console.log(`  ${cat}: ${amt.toFixed(2)}`);
        }
        console.log(`Всего транзакций: ${this.transactions.length}`);
    }

    budgetStatus() {
        if (Object.keys(this.budgets).length === 0 || this.transactions.length === 0) {
            console.log('Нет бюджетов или расходов.');
            return;
        }
        const spent = {};
        for (const t of this.transactions) {
            spent[t.category] = (spent[t.category] || 0) + t.amount;
        }
        console.log('📊 Бюджетный статус:');
        for (const [cat, budget] of Object.entries(this.budgets)) {
            const sp = spent[cat] || 0;
            const rem = budget - sp;
            const pct = budget > 0 ? (sp / budget) * 100 : 0;
            const sign = sp > budget ? '⚠️' : '✅';
            console.log(`  ${sign} ${cat}: потрачено ${sp.toFixed(2)}/${budget.toFixed(2)} (${pct.toFixed(1)}%), осталось ${rem.toFixed(2)}`);
        }
    }

    recommendations() {
        if (this.transactions.length === 0 || Object.keys(this.budgets).length === 0) {
            console.log('Добавьте расходы и бюджеты для получения рекомендаций.');
            return;
        }
        const spent = {};
        for (const t of this.transactions) {
            spent[t.category] = (spent[t.category] || 0) + t.amount;
        }
        const over = Object.entries(this.budgets).some(([cat, budget]) => (spent[cat] || 0) > budget);
        console.log('💡 Рекомендации:');
        if (over) console.log('  ⚠️ Вы превысили бюджет по некоторым категориям.');
        else console.log('  ✅ Все бюджеты в норме, отлично!');
        if (this.transactions.length < 5) console.log('  📝 Добавьте больше транзакций для точного анализа.');
    }

    exportCSV(filename = 'finance_export.csv') {
        let csv = 'Date,Category,Amount,Comment\n';
        for (const t of this.transactions) {
            csv += `${t.date},${t.category},${t.amount.toFixed(2)},${t.comment}\n`;
        }
        fs.writeFileSync(filename, csv);
        console.log(`Экспортировано в ${filename}`);
    }

    interactive() {
        console.log('💰 MoneyTracker Pro — JavaScript Edition');
        console.log('Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>,');
        console.log('        stats, status, recs, export, exit');
        rl.prompt();

        rl.on('line', (line) => {
            const parts = line.trim().split(' ');
            const cmd = parts[0];
            if (cmd === 'exit') {
                rl.close();
                return;
            }
            if (cmd === 'add') {
                if (parts.length >= 3) {
                    const category = parts[1];
                    const amount = parseFloat(parts[2]);
                    if (!isNaN(amount)) {
                        const comment = parts.slice(3).join(' ');
                        this.addTransaction(category, amount, comment);
                    } else {
                        console.log('Ошибка ввода суммы.');
                    }
                } else {
                    console.log('Использование: add <категория> <сумма> [комментарий]');
                }
            } else if (cmd === 'budget') {
                if (parts.length >= 3) {
                    const category = parts[1];
                    const budget = parseFloat(parts[2]);
                    if (!isNaN(budget)) {
                        this.setBudget(category, budget);
                    } else {
                        console.log('Ошибка ввода суммы.');
                    }
                } else {
                    console.log('Использование: budget <категория> <сумма>');
                }
            } else if (cmd === 'stats') {
                this.stats();
            } else if (cmd === 'status') {
                this.budgetStatus();
            } else if (cmd === 'recs') {
                this.recommendations();
            } else if (cmd === 'export') {
                const fname = parts.length > 1 ? parts[1] : 'finance_export.csv';
                this.exportCSV(fname);
            } else {
                console.log('Неизвестная команда.');
            }
            rl.prompt();
        }).on('close', () => {
            console.log('До свидания!');
            process.exit(0);
        });
    }
}

const fm = new FinanceManager();
fm.interactive();
