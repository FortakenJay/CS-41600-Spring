echo "Building and running cmd/bubble/main.go ..."
go build -o bubble cmd/bubble/main.go
chmod +x bubble
./bubble < sample_in.txt > bubble_out.txt
