#include "http_tcpServer.h"

#include <iostream>
#include <sstream>
#include <string>
#include <unistd.h>
#include <arpa/inet.h>

namespace {
const int BUFFER_SIZE = 30720;
const int LISTEN_BACKLOG = 20;

void log(const std::string &message) {
    std::cout << message << std::endl;
}

void exitWithError(const std::string &errorMessage) {
    log("Error: " + errorMessage);
    exit(EXIT_FAILURE);
}
} // namespace

namespace http {

TcpServer::TcpServer(std::string ip_address, int port)
    : m_ip_address(std::move(ip_address)), m_port(port), m_socket(-1), m_new_socket(-1),
      m_socketAddress(), m_socketAddress_len(sizeof(m_socketAddress)),
      m_serverMessage(buildResponse()) {

    m_socketAddress.sin_family = AF_INET;
    m_socketAddress.sin_port = htons(m_port);

    if (inet_pton(AF_INET, m_ip_address.c_str(), &m_socketAddress.sin_addr) <= 0) {
        exitWithError("Invalid IP address format");
    }

    if (startServer() != 0) {
        std::ostringstream ss;
        ss << "Failed to start server with PORT: " << ntohs(m_socketAddress.sin_port);
        log(ss.str());
    }
}

TcpServer::~TcpServer() {
    closeServer();
}

int TcpServer::startServer() {
    m_socket = socket(AF_INET, SOCK_STREAM, 0);
    if (m_socket < 0) {
        exitWithError("Cannot create socket");
    }

    if (bind(m_socket, (sockaddr *)&m_socketAddress, m_socketAddress_len) < 0) {
        exitWithError("Cannot connect socket to address");
    }

    return 0;
}

void TcpServer::closeServer() {
    if (m_socket >= 0) close(m_socket);
    if (m_new_socket >= 0) close(m_new_socket);
}

void TcpServer::startListen() {
    if (listen(m_socket, LISTEN_BACKLOG) < 0) {
        exitWithError("Socket listen failed");
    }

    std::ostringstream ss;
    ss << "\n*** Listening on ADDRESS: " << m_ip_address
       << " PORT: " << ntohs(m_socketAddress.sin_port) << " ***\n\n";
    log(ss.str());

    while (true) {
        log("====== Waiting for a new connection ======\n\n\n");
        acceptConnection(m_new_socket);

        char buffer[BUFFER_SIZE] = {0};
        int bytesReceived = read(m_new_socket, buffer, BUFFER_SIZE);
        if (bytesReceived < 0) {
            exitWithError("Failed to read bytes from client socket connection");
        }

        log("------ Received Request from client ------\n\n");

        sendResponse();

        close(m_new_socket);
    }
}

void TcpServer::acceptConnection(int &new_socket) {
    new_socket = accept(m_socket, nullptr, nullptr);
    if (new_socket < 0) {
        exitWithError("Failed to accept incoming connection");
    }
}

std::string TcpServer::buildResponse() {
    const std::string htmlFile = R"(
        <!DOCTYPE html>
        <html lang="en">
        <body>
            <h1> WHAT </h1>
            <p> Hello! from your Server (O.o) </p>
        </body>
        </html>
    )";
    std::ostringstream ss;
    ss << "HTTP/1.1 200 OK\r\n"
       << "Content-Type: text/html\r\n"
       << "Content-Length: " << htmlFile.size() << "\r\n\r\n"
       << htmlFile;

    return ss.str();
}

void TcpServer::sendResponse() {
    ssize_t bytesSent = write(m_new_socket, m_serverMessage.c_str(), m_serverMessage.size());

    if (bytesSent == static_cast<ssize_t>(m_serverMessage.size())) {
        log("------ Server Response sent to client -----\n\n");
    } else {
        log("Error sending response to client");
    }
}

} // namespace http
