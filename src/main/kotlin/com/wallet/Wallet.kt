package com.base

import io.javalin.Javalin
import io.javalin.http.staticfiles.Location

fun main() {
    // Initialize CSV database
    CsvDatabase.initializeCsv()

    val app = Javalin.create { config ->
        // location of static files
        config.staticFiles.add("/static", Location.CLASSPATH)
    }.start(8080)

    app.get("/") { ctx ->
        ctx.redirect("/home.html")
    }

    app.post("/create-account") { ctx ->
        val username = ctx.formParam("username")
        val phone = ctx.formParam("phone")
        val email = ctx.formParam("email")

        if (username.isNullOrBlank() || phone.isNullOrBlank() || email.isNullOrBlank()) {
            ctx.status(400).json(mapOf("message" to "All fields are required."))
            return@post
        }

        // Additional validation can go here...

        try {
            // Trowing an IOException
            CsvDatabase.writeUserToCsv(username, phone, email)
            ctx.status(201).json(mapOf("message" to "Account created successfully"))
        } catch (e: Exception) {
            ctx.status(500).json(mapOf("message" to "An error occurred while creating the account."))
        }
    }

     // add routes and logic

    // Register a shutdown hook
    Runtime.getRuntime().addShutdownHook(Thread {
        println("Shutdown signal received.")
        app.stop() // Stops the Javalin server
        // Add cleanup code here
    })

    // The application will continue running here until shutdown.
}

fun startTelegramBot() {
    // TDLib initialization and authentication
    // ...

    // Infinite loop to listen for updates from the Telegram API
    // ...
}

// Dapp class below
class Dapp {
    // Dapp class implementation
}

