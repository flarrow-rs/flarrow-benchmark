
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
        <script type="text/javascript" src="https://cdn.jsdelivr.net/npm/chart.xkcd@1.1/dist/chart.xkcd.min.js"></script>
</head>
<body>
    <div id="d5e26c0237594eef89ca5bedda5c3f31" class="chart-container" style="width: 800px">
        <svg id="chart_d5e26c0237594eef89ca5bedda5c3f31"></svg>
    </div>
    <script>
        const svg_d5e26c0237594eef89ca5bedda5c3f31 = document.querySelector('#chart_d5e26c0237594eef89ca5bedda5c3f31')
        const chart_d5e26c0237594eef89ca5bedda5c3f31 = new chartXkcd.Line(svg_d5e26c0237594eef89ca5bedda5c3f31, {"title": "Latencies", "data": {"datasets": [{"label": "raw dyn-static", "data": [118, 125, 116, 125, 119, 118, 123, 121, 116, 130]}, {"label": "wrapped static-dyn", "data": [162, 165, 160, 162, 153, 156, 156, 158, 156, 163]}, {"label": "wrapped static-static", "data": [38, 42, 42, 40, 39, 42, 39, 38, 40, 42]}, {"label": "raw static-dyn", "data": [143, 149, 139, 119, 119, 120, 121, 124, 114, 126]}, {"label": "raw static-static", "data": [27, 30, 29, 28, 30, 31, 28, 29, 31, 35]}, {"label": "wrapped dyn-dyn", "data": [185, 181, 180, 187, 204, 195, 214, 197, 200, 177]}, {"label": "wrapped dyn-static", "data": [195, 204, 196, 206, 203, 201, 195, 197, 192, 182]}, {"label": "raw dyn-dyn", "data": [161, 160, 132, 140, 139, 123, 114, 119, 120, 127]}], "labels": [1, 8, 64, 512, 2048, 4096, 16384, 40960, 409600, 4096000]}, "xLabel": "Size of the Payload", "yLabel": "Latency (\u00b5s)", "options": {"yTickCount": 3, "legendPosition": 1}});
    </script>
</body>
</html>

<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
        <script type="text/javascript" src="https://cdn.jsdelivr.net/npm/chart.xkcd@1.1/dist/chart.xkcd.min.js"></script>
</head>
<body>
    <div id="0540a9ef54e640ec806fb1c8dda474b2" class="chart-container" style="width: 800px">
        <svg id="chart_0540a9ef54e640ec806fb1c8dda474b2"></svg>
    </div>
    <script>
        const svg_0540a9ef54e640ec806fb1c8dda474b2 = document.querySelector('#chart_0540a9ef54e640ec806fb1c8dda474b2')
        const chart_0540a9ef54e640ec806fb1c8dda474b2 = new chartXkcd.Line(svg_0540a9ef54e640ec806fb1c8dda474b2, {"title": "Throughputs", "data": {"datasets": [{"label": "raw dyn-static", "data": [289192, 303216, 352310, 231606, 320741, 251942, 46580, 187260, 26342, 2540]}, {"label": "wrapped static-dyn", "data": [83730, 79275, 90516, 30328, 63604, 72236, 64232, 101330, 10925, 2223]}, {"label": "wrapped static-static", "data": [356344, 343355, 281018, 268780, 298352, 259241, 240798, 263632, 249789, 284762]}, {"label": "raw static-dyn", "data": [327266, 313783, 329601, 282851, 228101, 243129, 51938, 159176, 23316, 2576]}, {"label": "raw static-static", "data": [708105, 660419, 621705, 657384, 632971, 669685, 640045, 618215, 578031, 9410]}, {"label": "wrapped dyn-dyn", "data": [60094, 58902, 48725, 40068, 40416, 25850, 62374, 92207, 19988, 2184]}, {"label": "wrapped dyn-static", "data": [36786, 55864, 51333, 41931, 45517, 51444, 54034, 89403, 16525, 2261]}, {"label": "raw dyn-dyn", "data": [345342, 305285, 296805, 200840, 168505, 205215, 57107, 163933, 27639, 2573]}], "labels": [1, 8, 64, 512, 2048, 4096, 16384, 40960, 409600, 4096000]}, "xLabel": "Size of the Payload", "yLabel": "Throughput frequency (s\u207b\u00b9)", "options": {"yTickCount": 3, "legendPosition": 1}});
    </script>
</body>
</html>
