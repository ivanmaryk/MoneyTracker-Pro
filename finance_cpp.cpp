// finance_cpp.cpp — финансовый учёт с категориями расходов на C++

#include <iostream>
#include <vector>
#include <string>
#include <map>
#include <fstream>
#include <sstream>
#include <iomanip>
#include <algorithm>
#include <ctime>
#include <cmath>

using namespace std;

struct Transaction {
    string date;
    string category;
    double amount;
    string comment;
};

class FinanceManager {
private:
    vector<Transaction> transactions;
    map<string, double> budgets;
    vector<string> categories;
    string dataFile = "finance_data.csv";

    string currentDate() {
        time_t now = time(nullptr);
        char buf[20];
        strftime(buf, sizeof(buf), "%Y-%m-%d %H:%M:%S", localtime(&now));
        return string(buf);
    }

    void loadData() {
        ifstream file(dataFile);
        if (!file.is_open()) return;
        string line;
        // Формат: date,category,amount,comment
        while (getline(file, line)) {
            stringstream ss(line);
            Transaction t;
            string token;
            getline(ss, t.date, ',');
            getline(ss, t.category, ',');
            getline(ss, token, ','); t.amount = stod(token);
            getline(ss, t.comment, ',');
            transactions.push_back(t);
            if (find(categories.begin(), categories.end(), t.category) == categories.end())
                categories.push_back(t.category);
        }
        file.close();
        // Загрузка бюджетов (упрощённо, из отдельного файла)
        ifstream budgetFile("budgets.csv");
        if (budgetFile.is_open()) {
            string line;
            while (getline(budgetFile, line)) {
                stringstream ss(line);
                string cat;
                double bgt;
                getline(ss, cat, ',');
                ss >> bgt;
                budgets[cat] = bgt;
            }
            budgetFile.close();
        }
    }

    void saveData() {
        ofstream file(dataFile);
        if (!file.is_open()) return;
        for (const auto& t : transactions) {
            file << t.date << "," << t.category << "," << t.amount << "," << t.comment << "\n";
        }
        file.close();
        // Сохранение бюджетов
        ofstream budgetFile("budgets.csv");
        if (budgetFile.is_open()) {
            for (const auto& [cat, bgt] : budgets) {
                budgetFile << cat << "," << bgt << "\n";
            }
            budgetFile.close();
        }
    }

public:
    FinanceManager() { loadData(); }

    void addTransaction(const string& category, double amount, const string& comment = "") {
        if (find(categories.begin(), categories.end(), category) == categories.end())
            categories.push_back(category);
        Transaction t{currentDate(), category, amount, comment};
        transactions.push_back(t);
        saveData();
        cout << "✅ Расход добавлен: " << category << " " << fixed << setprecision(2) << amount << " (" << comment << ")" << endl;
    }

    void setBudget(const string& category, double budget) {
        budgets[category] = budget;
        saveData();
        cout << "✅ Бюджет для " << category << ": " << fixed << setprecision(2) << budget << endl;
    }

    void stats() {
        if (transactions.empty()) { cout << "Нет данных." << endl; return; }
        double total = 0;
        map<string, double> byCategory;
        for (const auto& t : transactions) {
            total += t.amount;
            byCategory[t.category] += t.amount;
        }
        cout << "📊 Общие расходы: " << fixed << setprecision(2) << total << endl;
        cout << "По категориям:" << endl;
        for (const auto& [cat, amt] : byCategory) {
            cout << "  " << cat << ": " << amt << endl;
        }
        cout << "Всего транзакций: " << transactions.size() << endl;
    }

    void budgetStatus() {
        if (budgets.empty() || transactions.empty()) {
            cout << "Нет бюджетов или расходов." << endl;
            return;
        }
        map<string, double> spent;
        for (const auto& t : transactions) spent[t.category] += t.amount;
        cout << "📊 Бюджетный статус:" << endl;
        for (const auto& [cat, budget] : budgets) {
            double sp = spent[cat];
            double rem = budget - sp;
            double pct = (budget > 0) ? (sp / budget) * 100 : 0;
            string sign = (sp > budget) ? "⚠️" : "✅";
            cout << "  " << sign << " " << cat << ": потрачено " << sp << "/" << budget
                 << " (" << pct << "%), осталось " << rem << endl;
        }
    }

    void recommendations() {
        if (transactions.empty() || budgets.empty()) {
            cout << "Добавьте расходы и бюджеты для получения рекомендаций." << endl;
            return;
        }
        bool over = false;
        map<string, double> spent;
        for (const auto& t : transactions) spent[t.category] += t.amount;
        for (const auto& [cat, budget] : budgets) {
            if (spent[cat] > budget) { over = true; break; }
        }
        cout << "💡 Рекомендации:" << endl;
        if (over) cout << "  ⚠️ Вы превысили бюджет по некоторым категориям." << endl;
        else cout << "  ✅ Все бюджеты в норме, отлично!" << endl;
        if (transactions.size() < 5) cout << "  📝 Добавьте больше транзакций для точного анализа." << endl;
    }

    void exportCSV(const string& filename = "finance_export.csv") {
        ofstream file(filename);
        if (!file.is_open()) return;
        file << "Date,Category,Amount,Comment\n";
        for (const auto& t : transactions) {
            file << t.date << "," << t.category << "," << t.amount << "," << t.comment << "\n";
        }
        file.close();
        cout << "Экспортировано в " << filename << endl;
    }

    void interactive() {
        cout << "💰 MoneyTracker Pro — C++ Edition" << endl;
        cout << "Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>," << endl;
        cout << "        stats, status, recs, export, exit" << endl;
        string cmd;
        while (true) {
            cout << "> ";
            getline(cin, cmd);
            if (cmd == "exit") break;
            else if (cmd.rfind("add", 0) == 0) {
                stringstream ss(cmd);
                string token, cat, comment;
                double amount;
                ss >> token >> cat >> amount;
                if (ss.fail()) {
                    cout << "Использование: add <категория> <сумма> [комментарий]" << endl;
                    continue;
                }
                getline(ss, comment);
                if (!comment.empty() && comment[0] == ' ') comment = comment.substr(1);
                addTransaction(cat, amount, comment);
            } else if (cmd.rfind("budget", 0) == 0) {
                stringstream ss(cmd);
                string token, cat;
                double budget;
                ss >> token >> cat >> budget;
                if (ss.fail()) {
                    cout << "Использование: budget <категория> <сумма>" << endl;
                    continue;
                }
                setBudget(cat, budget);
            } else if (cmd == "stats") {
                stats();
            } else if (cmd == "status") {
                budgetStatus();
            } else if (cmd == "recs") {
                recommendations();
            } else if (cmd == "export") {
                exportCSV();
            } else {
                cout << "Неизвестная команда." << endl;
            }
        }
    }
};

int main() {
    FinanceManager fm;
    fm.interactive();
    return 0;
}
