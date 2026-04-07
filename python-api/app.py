from flask import Flask, jsonify
import json

app = Flask(__name__)

DATA_FILE = "../data/output.json"

@app.route("/")
def home():
    return "API Running"

@app.route("/api/clusters")
def get_clusters():
    try:
        with open(DATA_FILE, "r") as f:
            data = json.load(f)
        return jsonify(data)
    except Exception as e:
        return jsonify({"error": str(e)})

if __name__ == "__main__":
    app.run(debug=True)