package main

import (
	"bufio"
	"crypto/rand"
	"encoding/csv"
	"encoding/json"
	"errors"
	"fmt"
	"math/big"
	"os"
	"strings"
	"time"

	"github.com/shopspring/decimal"
	"github.com/surrealdb/surrealdb.go"
	"golang.org/x/crypto/bcrypt"
)

const PASSWORD_DATABASE = "IE76qzUk0t78JGhTz"

type TransactionLog struct {
	Time   string `json:"time"`
	From   string `json:"from"`
	To     string `json:"to"`
	Amount string `json:"amount"`
}

type Account struct {
	ID           string `json:"id,omitempty"`
	Name         string `json:"name"`
	Balance      string `json:"balance"`
	Pin          string `json:"pin"`
	Transactions string `json:"transactions"`
}

type Transfer struct {
	From   string `json:"from"`
	To     string `json:"to"`
	Amount string `json:"amount"`
	Pin    string `json:"pin"`
}

func main() {

	fmt.Println("Welcome to the admin console")
	fmt.Println("the following commands are available:")
	fmt.Println("[1] create - create a new account")
	fmt.Println("[2] delete - delete an account")
	fmt.Println("[3] list - list all accounts")
	fmt.Println("[4] changepin - change the pin of an account")
	fmt.Println("[5] verify - verify an account")
	fmt.Println("[6] getlogs - get the logs of an account")
	fmt.Println("[7] transfer - transfer money between accounts")
	fmt.Println("[8] reversal - reverse a transaction")
	fmt.Println("[0] exit - exit the program")
	fmt.Println("Please enter the number of the command you would like to run:")
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Scan()
	if scanner.Err() != nil {
		fmt.Println(scanner.Err())
	}

	switch scanner := scanner; scanner.Text() {
	case "1":
		create()
	case "2":
		delete()
	case "3":
		listAll()
	case "4":
		changepin()
	case "5":
		verify()
	case "6":
		getlogs()
	case "7":
		transfer()
	case "8":
		reversal()
	case "0":
		os.Exit(0)
	}

	argsWithoutProg := os.Args[1:]
	arg := argsWithoutProg[0]
	switch arg {
	case "create":
		create()
	case "delete":
		delete()
	case "list":
		listAll()
	case "changepin":
		changepin()
	case "verify":
		verify()
	case "getlogs":
		getlogs()
	case "transfer":
		transfer()
	case "reversal":
		reversal()
	case "exit":
		os.Exit(0)
	}

}

func create() {
	var id string
	var name string
	var balance string
	var pin string
	fmt.Println("Enter Account ID:")
	fmt.Scanln(&id)
	fmt.Println("Enter Student Number:")
	fmt.Scanln(&name)
	fmt.Println("Enter Balance:")
	fmt.Scanln(&balance)
	fmt.Println("Enter Pin:")
	fmt.Scanln(&pin)

	var user Account
	createAccountStruct(&user, name, balance, id, pin)
	id, err := createAccount(user)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println("User created successfully with ID: " + id + " and Pin:" + user.Pin)
		fmt.Print("\n")
		main()

	}

}

func createAccountStruct(user *Account, name string, balance string, id string, pin string) {
	if id == "" {
		if pin == "" {
			userRef := user
			*userRef = Account{
				Name:         HashPassword(name),
				Balance:      balance,
				Pin:          HashPassword(randomPin()),
				Transactions: "",
			}
		} else {
			userRef := user
			*userRef = Account{
				Name:         HashPassword(name),
				Balance:      balance,
				Pin:          pin,
				Transactions: "",
			}
		}
	} else {
		if pin == "" {
			userRef := user
			*userRef = Account{
				ID:           id,
				Name:         HashPassword(name),
				Balance:      balance,
				Pin:          HashPassword(randomPin()),
				Transactions: "",
			}
		} else {
			userRef := user
			*userRef = Account{
				ID:           id,
				Name:         HashPassword(name),
				Balance:      balance,
				Pin:          HashPassword(pin),
				Transactions: "",
			}
		}
	}
}

func createAccount(user Account) (string, error) {
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	defer db.Close()
	if _, err := db.Use("user", "user"); err != nil {
		return "", err
	}
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		return "", fmt.Errorf("failed to sign in: %w", err)
	}
	// Insert user
	data, err := db.Select(user.ID)
	if err != nil {
		fmt.Print("error")
	}
	selectedUser := new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)
	if err == nil {
		return "", errors.New("User with ID: " + user.ID + " already exists.")
	}
	if selectedUser.ID != "" {
		return "", errors.New("User with ID: " + user.ID + " already exists.")
	}
	data, err = db.Create("user", user)
	if err != nil {
		return "", err
	}
	createdUser := make([]Account, 1)
	err = surrealdb.Unmarshal(data, &createdUser)
	if err != nil {
		return "", err
	}

	// Get user by ID
	data, err = db.Select(createdUser[0].ID)
	if err != nil {
		return "", err
	}
	selectedUser = new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)
	if err != nil {
		return "", err
	}
	if user.Name == selectedUser.Name {
		fmt.Println("User with ID: " + selectedUser.ID + " created successfully.")
		return selectedUser.ID, nil
	} else {
		return "", errors.New("failed to create user")
	}

}

func randomPin() string {
	max := big.NewInt(10000)
	n, _ := rand.Int(rand.Reader, max)
	return fmt.Sprintf("%04d", n)
}

func listAll() {
	db, _ := surrealdb.New("ws://localhost:8000/rpc")

	defer db.Close()
	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return
	}
	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	data, err := db.Query("SELECT id FROM user", map[string]interface{}{})
	if err != nil {
		panic(err)
	}
	jsonData, err := json.MarshalIndent(data, "", "  ")
	if err != nil {
		panic(err)
	}

	// Print the JSON data
	fmt.Println(string(jsonData))

	fmt.Print("\n")
	main()
}

func delete() {
	fmt.Println("Enter Account ID:")
	var id string
	fmt.Scanln(&id)
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return
	}
	defer db.Close()
	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	_, err := db.Delete("user:" + id)
	if err != nil {
		fmt.Println(err)
	} else {
		fmt.Println("User with ID: " + id + " deleted successfully.")
	}

	fmt.Print("\n")
	main()
}

func changepin() {
	var id string
	var pin string

	var name string
	fmt.Println("Enter Account ID:")
	fmt.Scanln(&id)
	fmt.Println("Enter Student Number:")
	fmt.Scanln(&name)
	fmt.Println("Enter New Pin:")
	fmt.Scanln(&pin)
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	defer db.Close()
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return
	}
	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	data, err := db.Select("user:" + id)
	if err != nil {
		fmt.Println(err)
	}
	selectedUser := new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)
	if err != nil {
		fmt.Println(err)
	}
	if CheckPasswordHash(name, selectedUser.Name) {
		fmt.Println("Verified")
	} else {
		fmt.Println("Error")
	}
	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	changes := map[string]string{"pin": HashPassword(pin), "name": selectedUser.Name, "balance": selectedUser.Balance, "transactions": selectedUser.Transactions}
	if _, err = db.Update(selectedUser.ID, changes); err != nil {
		panic(err)
	}

	fmt.Print("\n")
	main()
}

func verify() {
	var id string
	var name string
	fmt.Println("Enter Account ID:")
	fmt.Scanln(&id)
	fmt.Println("Enter Student Number:")
	fmt.Scanln(&name)
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return
	}
	defer db.Close()
	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	data, err := db.Select("user:" + id)
	if err != nil {
		fmt.Println(err)
	}
	selectedUser := new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)
	if err != nil {
		fmt.Println(err)
	}
	if CheckPasswordHash(name, selectedUser.Name) {
		fmt.Println("Verified")
	} else {
		fmt.Println("Error")
	}

	fmt.Print("\n")
	main()
}

func CheckPasswordHash(password, hash string) bool {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(password))
	return err == nil
}
func HashPassword(password string) string {
	bytes, _ := bcrypt.GenerateFromPassword([]byte(password), 14)
	return string(bytes)
}

func getlogs() {
	var id string
	fmt.Println("Enter Account ID:")
	fmt.Scanln(&id)
	logs, err := readLogs("user:" + id)
	if err != nil {
		panic(err)
	}
	fmt.Println(logs)
}

func readLogs(ID string) (string, error) {
	//fmt.Println(ID, PIN)
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	defer db.Close()
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return "", err
	}
	if _, err := db.Use("user", "user"); err != nil {
		return "", fmt.Errorf("failed to use database: %w", err)
	}
	data, err := db.Select(ID)
	if err != nil {
		return "", fmt.Errorf("failed to select account with ID %s: %w", ID, err)
	}
	acc1 := new(Account)
	err = surrealdb.Unmarshal(data, &acc1)
	if err != nil {
		return "", fmt.Errorf("failed to unmarshal account data: %w", err)
	}
	fmt.Println(acc1.Transactions)
	if acc1.Transactions == "" {
		return "", fmt.Errorf("no transactions found for account with ID %s", ID)
	} else {
		return acc1.Transactions, nil
	}

}

func transfer() {
	var from string
	var to string
	var amount string
	fmt.Println("Enter Account ID to transfer from:")
	fmt.Scanln(&from)
	fmt.Println("Enter Account ID to transfer to:")
	fmt.Scanln(&to)
	fmt.Println("Enter Amount:")
	fmt.Scanln(&amount)
	transferMoney(Transfer{From: "user:" + from, To: "user:" + to, Amount: amount, Pin: ""})
}

func transferMoney(transfer Transfer) error {
	from, err := loadUser(transfer.From)
	if err != nil {
		return fmt.Errorf("failed to load user with ID %s: %w", transfer.From, err)
	}
	to, err := loadUser(transfer.To)
	if err != nil {
		return fmt.Errorf("failed to load user with ID %s: %w", transfer.To, err)
	}
	err = validateTransaction(transfer)
	if err != nil {
		return fmt.Errorf("failed to validate transaction: %w", err)
	}
	transferDecimal, err := decimal.NewFromString(transfer.Amount)
	if err != nil {
		return fmt.Errorf("failed to convert transfer amount to decimal: %w", err)
	}

	//handle from
	updatedTransaction, err := appendToLog(from, transfer)
	if err != nil {
		return fmt.Errorf("failed to append transaction to log: %w", err)
	}
	from.Transactions = updatedTransaction.Transactions
	if updateUser(from.ID, from) != nil {
		return fmt.Errorf("failed to update user: %w", err)
	}
	//handle to
	toDecimal, err := decimal.NewFromString(to.Balance)
	if err != nil {
		return fmt.Errorf("failed to convert balance to decimal: %w", err)
	}
	to.Balance = toDecimal.Add(transferDecimal).String()
	updatedTransaction, err = appendToLog(to, transfer)
	if err != nil {
		return fmt.Errorf("failed to append transaction to log: %w", err)
	}
	to.Transactions = updatedTransaction.Transactions
	if updateUser(to.ID, to) != nil {
		return fmt.Errorf("failed to update user: %w", err)
	}
	return nil
}

func loadUser(id string) (Account, error) {
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	defer db.Close()
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return Account{}, err
	}

	if _, err := db.Use("user", "user"); err != nil {
		return Account{}, fmt.Errorf("failed to use database: %w", err)
	}
	data, err := db.Select(id)
	if err != nil {
		return Account{}, fmt.Errorf("failed to select account with ID %s: %w", id, err)
	}
	acc1 := new(Account)
	err = surrealdb.Unmarshal(data, &acc1)
	if err != nil {
		return Account{}, fmt.Errorf("failed to unmarshal account data: %w", err)
	}
	fmt.Println(acc1)
	return Account{
		ID:           acc1.ID,
		Name:         acc1.Name,
		Balance:      acc1.Balance,
		Pin:          acc1.Pin,
		Transactions: acc1.Transactions,
	}, nil
}

func validateTransaction(transfer Transfer) error {
	if transfer.From == transfer.To {
		return fmt.Errorf("same account")
	}
	transferDecimal, err := decimal.NewFromString(transfer.Amount)
	if err != nil {
		return fmt.Errorf("failed to convert transfer amount to decimal: %w", err)
	}
	if transferDecimal.LessThanOrEqual(decimal.Zero) {
		return fmt.Errorf("bad amount")
	}
	return nil
}

func updateUser(id string, acc Account) error {
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	defer db.Close()
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
		return err
	}

	if _, err := db.Use("user", "user"); err != nil {
		return fmt.Errorf("failed to use database: %w", err)
	}
	data, err := db.Select(id)
	if err != nil {
		return fmt.Errorf("failed to select account with ID %s: %w", id, err)
	}
	acc2 := new(Account)
	err = surrealdb.Unmarshal(data, &acc2)
	if err != nil {
		return fmt.Errorf("failed to unmarshal account data: %w", err)
	}

	changes := map[string]string{"balance": acc.Balance, "name": acc.Name, "pin": acc.Pin, "transactions": acc.Transactions}
	if _, err = db.Update(id, changes); err != nil {
		return fmt.Errorf("failed to update account with ID %s: %w", id, err)
	}
	return nil
}

func appendToLog(acc1 Account, transfer Transfer) (Account, error) {
	var transactions []TransactionLog
	if acc1.Transactions == "" {
		transactions = []TransactionLog{}
	} else {
		err := json.Unmarshal([]byte(acc1.Transactions), &transactions)
		if err != nil {
			return Account{}, fmt.Errorf("failed to unmarshal transactions: %w", err)
		}
	}
	transactions = append(transactions, TransactionLog{Time: currTime(), From: transfer.From, To: transfer.To, Amount: transfer.Amount})
	transactionsJSON, err := json.Marshal(transactions)
	if err != nil {
		return Account{}, fmt.Errorf("failed to marshal transactions: %w", err)
	}
	transactionsString := string(transactionsJSON)
	acc1.Transactions = transactionsString
	return acc1, nil
}

func reversal() {
	var amount string
	var from string
	var to string
	fmt.Println("Enter Account ID to reverse transaction from:")
	fmt.Scanln(&from)
	fmt.Println("Enter Account ID to reverse transaction to:")
	fmt.Scanln(&to)
	fmt.Println("Amount payed (Taxes included):")
	fmt.Scanln(&amount)
	reverseTransaction("user:"+from, "user:"+to, amount)
}

func reverseTransaction(from string, to string, amount string) {
	db, _ := surrealdb.New("ws://localhost:8000/rpc")
	defer db.Close()
	if _, err := db.Signin(map[string]interface{}{
		"user": "guffe",
		"pass": PASSWORD_DATABASE,
	}); err != nil {
		fmt.Println(err)
	}

	if _, err := db.Use("user", "user"); err != nil {
		fmt.Println(err)
	}
	data, err := db.Select(from)
	if err != nil {
		fmt.Println(err)
	}
	selectedUser := new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)
	if err != nil {
		fmt.Println(err)
	}
	balance, _ := decimal.NewFromString(selectedUser.Balance)
	amountDec, _ := decimal.NewFromString(amount)
	var transactions []TransactionLog
	if selectedUser.Transactions == "" {
		transactions = []TransactionLog{}
	} else {
		err = json.Unmarshal([]byte(selectedUser.Transactions), &transactions)
		if err != nil {
			return
		}
	}
	err = logfile(TransactionLog{Time: currTime(), From: from, To: to, Amount: amount})
	if err != nil {
		return
	}
	fromFormatted, _ := strings.CutPrefix(from, "user:")
	toFormatted, _ := strings.CutPrefix(to, "user:")
	transactions = append(transactions, TransactionLog{Time: currTime(), From: fromFormatted, To: toFormatted, Amount: amount})
	transactionsJSON, err := json.Marshal(transactions)
	if err != nil {
		return
	}
	transactionsString := string(transactionsJSON)

	changes := map[string]string{"pin": selectedUser.Pin, "name": selectedUser.Name, "balance": (balance.Add(amountDec).String()), "transactions": transactionsString}
	if _, err = db.Update(selectedUser.ID, changes); err != nil {
		panic(err)
	}
	data, err = db.Select(to)
	if err != nil {
		fmt.Println(err)
	}
	selectedUser = new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)
	if err != nil {
		fmt.Println(err)
	}
	balance, _ = decimal.NewFromString(selectedUser.Balance)
	amountToSubtract := amountDec.Div(decimal.NewFromFloat(1.1))
	if selectedUser.Transactions == "" {
		transactions = []TransactionLog{}
	} else {
		err = json.Unmarshal([]byte(selectedUser.Transactions), &transactions)
		if err != nil {
			return
		}
	}
	err = logfile(TransactionLog{Time: currTime(), From: from, To: to, Amount: amount})
	if err != nil {
		return
	}
	transactions = append(transactions, TransactionLog{Time: currTime(), From: fromFormatted, To: toFormatted, Amount: amount})
	change := map[string]string{"pin": selectedUser.Pin, "name": selectedUser.Name, "balance": balance.Sub(amountToSubtract).String(), "transactions": selectedUser.Transactions}
	if _, err = db.Update(selectedUser.ID, change); err != nil {
		panic(err)
	}
	data, err = db.Select("user:zentralbank")
	if err != nil {
		fmt.Println(err)
	}
	selectedUser = new(Account)
	err = surrealdb.Unmarshal(data, &selectedUser)

	if err != nil {
		fmt.Println(err)
	}
	balance, _ = decimal.NewFromString(selectedUser.Balance)
	if selectedUser.Transactions == "" {
		transactions = []TransactionLog{}
	} else {
		err = json.Unmarshal([]byte(selectedUser.Transactions), &transactions)
		if err != nil {
			return
		}
	}
	err = logfile(TransactionLog{Time: currTime(), From: from, To: to, Amount: amount})
	if err != nil {
		return
	}
	transactions = append(transactions, TransactionLog{Time: currTime(), From: fromFormatted, To: toFormatted, Amount: amount})
	amountToRemove := amountToSubtract.Mul(decimal.NewFromFloat(0.1))
	change = map[string]string{"pin": selectedUser.Pin, "name": selectedUser.Name, "balance": balance.Sub(amountToRemove).String(), "transactions": selectedUser.Transactions}
	if _, err = db.Update("user:zentralbank", change); err != nil {
		panic(err)
	}
	fmt.Println("Transaction reversed successfully")

}

func currTime() string {
	locat, error := time.LoadLocation("Euroe/Berlin")
	var dt time.Time
	if error != nil {
		dt = time.Now()
		fmt.Println(error)
	} else {
		dt = time.Now().In(locat)
	}
	return dt.Format("01-02-2006 15:04:05")
}

func logfile(transaction TransactionLog) error {
	//create file if it doesn't exist
	if _, err := os.Stat("transactions.csv"); errors.Is(err, os.ErrNotExist) {
		os.Create("transactions.csv")
		//create header
		file, err := os.OpenFile("transactions.csv", os.O_APPEND|os.O_WRONLY, 0644)
		if err != nil {
			return err
		}
		defer file.Close()
		writer := csv.NewWriter(file)
		defer writer.Flush()
		data := []string{"Time", "From", "To", "Amount"}
		err = writer.Write(data)
		if err != nil {
			return err
		}
	}
	file, err := os.OpenFile("transactions.csv", os.O_APPEND|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer file.Close()
	writer := csv.NewWriter(file)
	defer writer.Flush()
	data := []string{transaction.Time, strings.TrimPrefix(transaction.From, "user:"), strings.TrimPrefix(transaction.From, "user:"), transaction.Amount}
	err = writer.Write(data)
	if err != nil {
		return err
	}
	return nil
}
