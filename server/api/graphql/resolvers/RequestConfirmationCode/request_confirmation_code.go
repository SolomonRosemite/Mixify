package requestConfirmationCode

import (
	"context"
	"fmt"
	"log"
	"math/rand"
	"net/smtp"
	"os"
	"strings"
	"time"

	"github.com/SolomonRosemite/Mixify/api/graphql/model"
	"github.com/SolomonRosemite/Mixify/internal/models"
	"github.com/SolomonRosemite/Mixify/internal/utils/common"
)

func RequestConfirmationCode(ctx context.Context, email string, codes *map[string]*models.EmailConfirmation) (*model.RequestConfirmationCodeResponse, error) {
	email = strings.TrimSpace(email)
	random := rand.New(rand.NewSource(time.Now().UnixNano()))

	var confirmationCode string = generateRandomString(random, 4)
	var confirmationSecret string = generateRandomString(random, 10)

	for {
		// If the confirmation combination already exists, generate a new one
		if _, found := (*codes)[confirmationCode+confirmationSecret]; found {
			confirmationCode = generateRandomString(random, 4)
			confirmationSecret = generateRandomString(random, 10)
		} else {
			break
		}
	}

	(*codes)[confirmationCode+confirmationSecret] = &models.EmailConfirmation{
		Email:      &email,
		Expiration: common.LiteralToPtr(time.Now().Add(time.Minute * 5)),
	}

	go func() {
		sendConfirmationCodeToEmail(email, confirmationCode)
	}()

	return &model.RequestConfirmationCodeResponse{
		ConfirmationSecret: confirmationSecret,
	}, nil
}

func sendConfirmationCodeToEmail(targetEmail string, confirmationCode string) {
	from := os.Getenv("NOTIFICATION_EMAIL")
	pass := os.Getenv("NOTIFICATION_EMAIL_APP_PASSWORD")
	body := fmt.Sprintf("Hello there.\n\n This is your confirmation code for signing in: %s\n\nWe can't wait to see you on the other side", confirmationCode)

	msg := "From: " + from + " Mixify\n" +
		"To: " + targetEmail + "\n" +
		fmt.Sprintf("Subject: Your email confirmation code is: %s \n\n", confirmationCode) +
		body

	err := smtp.SendMail("smtp.gmail.com:587",
		smtp.PlainAuth("", from, pass, "smtp.gmail.com"),
		from, []string{targetEmail}, []byte(msg))

	if err != nil {
		log.Printf("failed to send email to %s. smtp error: %s", targetEmail, err)
		return
	}

	log.Printf("sent confirmation email to %s successfully", targetEmail)
}

func generateRandomString(r *rand.Rand, length int) string {
	const charset = "0123456789"

	b := make([]byte, length)
	for i := range b {
		b[i] = charset[r.Intn(len(charset))]
	}
	return string(b)
}
