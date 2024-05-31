import com.pengrad.telegrambot.TelegramBot
import com.pengrad.telegrambot.UpdatesListener
import com.pengrad.telegrambot.request.SendMessage

fun main() {
    // Fetch the bot token from an environment variable
    val botToken = System.getenv("TELEGRAM_BOT_TOKEN") ?: throw IllegalArgumentException("Please set the TELEGRAM_BOT_TOKEN environment variable.")
    
    val bot = TelegramBot(botToken) // Use the token from the environment variable

    bot.setUpdatesListener { updates ->
        // ... process updates
        // For example, send an echo message back
        updates.forEach { update ->
            update.message()?.let { message ->
                val chatId = message.chat().id()
                val text = message.text()
                bot.execute(SendMessage(chatId, text)) // Echo the received message
            }
        }
        UpdatesListener.CONFIRMED_UPDATES_ALL
    }
}
