#include "http_tcpServer.h"

#include <iostream>

int main() {
  std::cout << "---- Starting MyHTTP Server ----\n";

  using namespace http;
  TcpServer server = TcpServer("0.0.0.0", 8080);
  server.startListen();

  return 0;
}
