import express from "express";
import { fileURLToPath } from "url";
import { dirname, join } from "path";
import { readFile } from "fs/promises";

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const app = express();

// Enable CORS
app.use((req, res, next) => {
  res.header("Access-Control-Allow-Origin", "*");
  res.header(
    "Access-Control-Allow-Headers",
    "Origin, X-Requested-With, Content-Type, Accept"
  );
  next();
});

// Serve static files with correct MIME types
app.use(express.static(__dirname));

// Special handling for .wasm files
app.get("*.wasm", async (req, res) => {
  try {
    const wasmPath = join(__dirname, req.path);
    const wasmBuffer = await readFile(wasmPath);
    res.set({
      "Content-Type": "application/wasm",
      "Content-Length": wasmBuffer.length,
    });
    res.send(wasmBuffer);
  } catch (error) {
    res.status(404).send("Not found");
  }
});

// Special handling for .js files
app.get("*.js", async (req, res) => {
  try {
    const jsPath = join(__dirname, req.path);
    const jsBuffer = await readFile(jsPath);
    res.set("Content-Type", "application/javascript");
    res.send(jsBuffer);
  } catch (error) {
    res.status(404).send("Not found");
  }
});

const PORT = 3000;
app.listen(PORT, () => {
  console.log(`Server running at http://localhost:${PORT}`);
});
