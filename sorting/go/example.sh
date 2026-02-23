echo "building and running cmd/bubble/bubble.go"
go build cmd/bubble/bubble.go -o cmd/bubble/bubble
cmd/bubble/bubble < sample_in.txt > bubble_out.txt
