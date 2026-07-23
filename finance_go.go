// finance_go.go — финансовый учёт с категориями расходов на Go

package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
	"time"
)

type Transaction struct {
	Date     string  `json:"date"`
	Category string  `json:"category"`
	Amount   float64 `json:"amount"`
	Comment  string  `json:"comment"`
}

type FinanceManager struct {
	transactions []Transaction
	budgets      map[string]float64
	categories   map[string]bool
	dataFile     string
}

func NewFinanceManager() *FinanceManager {
	fm := &FinanceManager{
		budgets:    make(map[string]float64),
		categories: make(map[string]bool),
		dataFile:   "finance_data.json",
	}
	fm.loadData()
	return fm
}

func (fm *FinanceManager) loadData() {
	data, err := ioutil.ReadFile(fm.dataFile)
	if err != nil {
		return
	}
	var raw map[string]interface{}
	json.Unmarshal(data, &raw)
	if raw == nil {
		return
	}
	if t, ok := raw["transactions"].([]interface{}); ok {
		for _, v := range t {
			if m, ok := v.(map[string]interface{}); ok {
				tr := Transaction{
					Date:     m["date"].(string),
					Category: m["category"].(string),
					Amount:   m["amount"].(float64),
					Comment:  m["comment"].(string),
				}
				fm.transactions = append(fm.transactions, tr)
				fm.categories[tr.Category] = true
			}
		}
	}
	if b, ok := raw["budgets"].(map[string]interface{}); ok {
		for k, v := range b {
			fm.budgets[k] = v.(float64)
		}
	}
}

func (fm *FinanceManager) saveData() {
	data := map[string]interface{}{
		"transactions": fm.transactions,
		"budgets":      fm.budgets,
		"categories":   fm.categories,
	}
	jsonData, _ := json.MarshalIndent(data, "", "  ")
	ioutil.WriteFile(fm.dataFile, jsonData, 0644)
}

func (fm *FinanceManager) addTransaction(category string, amount float64, comment string) {
	if !fm.categories[category] {
		fm.categories[category] = true
	}
	tr := Transaction{
		Date:     time.Now().Format(time.RFC3339),
		Category: category,
		Amount:   amount,
		Comment:  comment,
	}
	fm.transactions = append(fm.transactions, tr)
	fm.saveData()
	fmt.Printf("✅ Расход добавлен: %s %.2f (%s)\n", category, amount, comment)
}

func (fm *FinanceManager) setBudget(category string, budget float64) {
	fm.budgets[category] = budget
	fm.saveData()
	fmt.Printf("✅ Бюджет для %s: %.2f\n", category, budget)
}

func (fm *FinanceManager) stats() {
	if len(fm.transactions) == 0 {
		fmt.Println("Нет данных.")
		return
	}
	total := 0.0
	byCat := make(map[string]float64)
	for _, t := range fm.transactions {
		total += t.Amount
		byCat[t.Category] += t.Amount
	}
	fmt.Printf("📊 Общие расходы: %.2f\n", total)
	fmt.Println("По категориям:")
	for cat, amt := range byCat {
		fmt.Printf("  %s: %.2f\n", cat, amt)
	}
	fmt.Printf("Всего транзакций: %d\n", len(fm.transactions))
}

func (fm *FinanceManager) budgetStatus() {
	if len(fm.budgets) == 0 || len(fm.transactions) == 0 {
		fmt.Println("Нет бюджетов или расходов.")
		return
	}
	spent := make(map[string]float64)
	for _, t := range fm.transactions {
		spent[t.Category] += t.Amount
	}
	fmt.Println("📊 Бюджетный статус:")
	for cat, budget := range fm.budgets {
		sp := spent[cat]
		rem := budget - sp
		pct := 0.0
		if budget > 0 {
			pct = (sp / budget) * 100
		}
		sign := "✅"
		if sp > budget {
			sign = "⚠️"
		}
		fmt.Printf("  %s %s: потрачено %.2f/%.2f (%.1f%%), осталось %.2f\n", sign, cat, sp, budget, pct, rem)
	}
}

func (fm *FinanceManager) recommendations() {
	if len(fm.transactions) == 0 || len(fm.budgets) == 0 {
		fmt.Println("Добавьте расходы и бюджеты для получения рекомендаций.")
		return
	}
	spent := make(map[string]float64)
	for _, t := range fm.transactions {
		spent[t.Category] += t.Amount
	}
	over := false
	for cat, budget := range fm.budgets {
		if spent[cat] > budget {
			over = true
			break
		}
	}
	fmt.Println("💡 Рекомендации:")
	if over {
		fmt.Println("  ⚠️ Вы превысили бюджет по некоторым категориям.")
	} else {
		fmt.Println("  ✅ Все бюджеты в норме, отлично!")
	}
	if len(fm.transactions) < 5 {
		fmt.Println("  📝 Добавьте больше транзакций для точного анализа.")
	}
}

func (fm *FinanceManager) exportCSV(filename string) {
	file, _ := os.Create(filename)
	defer file.Close()
	file.WriteString("Date,Category,Amount,Comment\n")
	for _, t := range fm.transactions {
		file.WriteString(fmt.Sprintf("%s,%s,%.2f,%s\n", t.Date, t.Category, t.Amount, t.Comment))
	}
	fmt.Printf("Экспортировано в %s\n", filename)
}

func (fm *FinanceManager) interactive() {
	scanner := bufio.NewScanner(os.Stdin)
	fmt.Println("💰 MoneyTracker Pro — Go Edition")
	fmt.Println("Команды: add <категория> <сумма> [комментарий], budget <категория> <сумма>,")
	fmt.Println("        stats, status, recs, export, exit")
	for {
		fmt.Print("> ")
		if !scanner.Scan() {
			break
		}
		line := strings.TrimSpace(scanner.Text())
		if line == "" {
			continue
		}
		parts := strings.Fields(line)
		cmd := parts[0]
		switch cmd {
		case "exit":
			return
		case "add":
			if len(parts) >= 3 {
				cat := parts[1]
				amount, err := strconv.ParseFloat(parts[2], 64)
				if err == nil {
					comment := strings.Join(parts[3:], " ")
					fm.addTransaction(cat, amount, comment)
				} else {
					fmt.Println("Ошибка ввода суммы.")
				}
			} else {
				fmt.Println("Использование: add <категория> <сумма> [комментарий]")
			}
		case "budget":
			if len(parts) >= 3 {
				cat := parts[1]
				budget, err := strconv.ParseFloat(parts[2], 64)
				if err == nil {
					fm.setBudget(cat, budget)
				} else {
					fmt.Println("Ошибка ввода суммы.")
				}
			} else {
				fmt.Println("Использование: budget <категория> <сумма>")
			}
		case "stats":
			fm.stats()
		case "status":
			fm.budgetStatus()
		case "recs":
			fm.recommendations()
		case "export":
			fm.exportCSV("finance_export.csv")
		default:
			fmt.Println("Неизвестная команда.")
		}
	}
}

func main() {
	NewFinanceManager().interactive()
}
