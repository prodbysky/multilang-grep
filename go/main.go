package main

import (
	"bufio"
	"errors"
	"fmt"
	"log"
	"os"
	"strings"
)

func main() {
	config, e := ConfigNew(os.Args)

	if e != nil {
		log.Fatalf("%s", e)
		return
	}

	file_content_bytes, e := os.ReadFile(config.file_name)
	if e != nil {
		log.Fatalf("%s", e)
		return
	}

	file_content := string(file_content_bytes)

	scanner := bufio.NewScanner(strings.NewReader(file_content))

	for scanner.Scan() {
		if strings.Contains(scanner.Text(), config.needle) {
			fmt.Println(scanner.Text())
		}
	}
}

type Config struct {
	file_name string
	needle    string
}

func ConfigNew(args []string) (Config, error) {
	if len(args) < 3 {
		return Config{}, errors.New("Not enough arguments provided")
	}
	file_name := args[1]
	needle := args[2]

	return Config{file_name, needle}, nil
}
