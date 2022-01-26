import matplotlib

matplotlib.use("module://mplcairo.tk")

import matplotlib.pyplot as pyplot

# pyplot.switch_backend("qtcairo")

# Plot 3D scatter
figure = pyplot.figure()
plot = figure.add_subplot(111, projection='3d')

# for i in range(len(raw_data)):
    # xraw = raw_data[i][0]
    # yraw = raw_data[i][1]
    # zraw = raw_data[i][2]

#     xcalib = calibData[i, 0]
#     ycalib = calibData[i, 1]
#     zcalib = calibData[i, 2]
    # plot.scatter(xraw, yraw, zraw, color='r')
#     plot.scatter(xcalib, ycalib, zcalib, color='b')

plot.set_title('3D Scatter Plot of Magnetometer Data')
plot.set_xlabel('X [uT]')
plot.set_ylabel('Y [uT]')
plot.set_zlabel('Z [uT]')


pyplot.show()