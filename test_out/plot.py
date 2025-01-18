import csv
import matplotlib.pyplot as plt

frequencies = []
time = []

with open('test_out/frequencies.csv', 'r') as file:
    csv_reader = csv.reader(file)
    i = 0        
    for row in csv_reader:
        frequencies.append(row[0])
        time.append(i)
        i += 1
        

plt.scatter(time, frequencies)
plt.xlabel('Time')
plt.ylabel('Frequency (Hz)')
plt.show()