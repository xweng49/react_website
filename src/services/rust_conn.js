const fetch = require("node-fetch");

data = {
	"name": "AAPL",
	"date_start": "1-1-2020",
	"date_end": "1-1-2021"
}



var obj
fetch("http://127.0.0.1:8080/mjsonrust", {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
})
.then(res => res.json())
.then(data => obj = data)
.then(() => console.log(obj))
