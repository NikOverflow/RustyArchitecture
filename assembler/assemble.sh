if [ $# -ne 0 ]; then
  customasm -f binary "$1" -o "$1.bin"
  #customasm -f hexstr -pq $1 &> /dev/null
  #if [ $? -eq 0 ]; then
    #customasm -f hexstr -pq $1 | fold -w 16 | sed -e 's/^/0x/'
  #else
    #customasm -f hexstr -pq $1
  #fi
else
  echo "Please specify a File!"
fi