import socket

# Server details
host = 'localhost'  # Change this to the IP address or hostname of the server
port = 57575  # The port you want to connect to

# Data to be sent
data_to_send = "Moin\0"

# Create a socket object
client_socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Connect to the server
server_address = (host, port)
client_socket.connect(server_address)

# Send data
client_socket.sendall(data_to_send.encode('utf-8'))
# Close the connection
client_socket.close()
