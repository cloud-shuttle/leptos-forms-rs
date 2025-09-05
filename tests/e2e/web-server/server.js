#!/usr/bin/env node

const http = require("http");
const fs = require("fs");
const path = require("path");
const url = require("url");

const PORT = process.env.PORT || 3000;
const STATIC_DIR = __dirname;

// MIME types for different file extensions
const MIME_TYPES = {
  ".html": "text/html",
  ".css": "text/css",
  ".js": "application/javascript",
  ".json": "application/json",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".gif": "image/gif",
  ".svg": "image/svg+xml",
  ".ico": "image/x-icon",
  ".txt": "text/plain",
  ".pdf": "application/pdf",
  ".zip": "application/zip",
  ".mp4": "video/mp4",
  ".mp3": "audio/mpeg",
};

// Default to index.html for root path
function getFilePath(requestPath) {
  if (requestPath === "/") {
    return path.join(STATIC_DIR, "index.html");
  }

  // Handle specific routes
  if (requestPath === "/components") {
    return path.join(STATIC_DIR, "components.html");
  }

  // Remove leading slash and resolve path
  const filePath = path.join(STATIC_DIR, requestPath);

  // Security check: ensure the resolved path is within the static directory
  const resolvedPath = path.resolve(filePath);
  if (!resolvedPath.startsWith(path.resolve(STATIC_DIR))) {
    return null; // Path traversal attempt
  }

  return filePath;
}

// Get MIME type based on file extension
function getMimeType(filePath) {
  const ext = path.extname(filePath).toLowerCase();
  return MIME_TYPES[ext] || "application/octet-stream";
}

// Create HTTP server
const server = http.createServer((req, res) => {
  const parsedUrl = url.parse(req.url);
  const requestPath = parsedUrl.pathname;

  console.log(`${new Date().toISOString()} - ${req.method} ${requestPath}`);

  // Handle CORS for testing
  res.setHeader("Access-Control-Allow-Origin", "*");
  res.setHeader(
    "Access-Control-Allow-Methods",
    "GET, POST, PUT, DELETE, OPTIONS",
  );
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Authorization");

  if (req.method === "OPTIONS") {
    res.writeHead(200);
    res.end();
    return;
  }

  const filePath = getFilePath(requestPath);

  if (!filePath) {
    res.writeHead(403, { "Content-Type": "text/plain" });
    res.end("Forbidden");
    return;
  }

  // Check if file exists
  if (!fs.existsSync(filePath)) {
    res.writeHead(404, { "Content-Type": "text/html" });
    res.end(`
            <!DOCTYPE html>
            <html>
            <head><title>404 - Not Found</title></head>
            <body>
                <h1>404 - Not Found</h1>
                <p>The requested file "${requestPath}" was not found.</p>
                <p><a href="/">Go back to index</a></p>
            </body>
            </html>
        `);
    return;
  }

  // Check if it's a directory
  const stat = fs.statSync(filePath);
  if (stat.isDirectory()) {
    // Try to serve index.html from directory
    const indexPath = path.join(filePath, "index.html");
    if (fs.existsSync(indexPath)) {
      serveFile(indexPath, res);
    } else {
      // List directory contents
      res.writeHead(200, { "Content-Type": "text/html" });
      const files = fs.readdirSync(filePath);
      const fileList = files
        .map((file) => {
          const isDir = fs.statSync(path.join(filePath, file)).isDirectory();
          const icon = isDir ? "📁" : "📄";
          return `<li><a href="${path.join(requestPath, file)}">${icon} ${file}</a></li>`;
        })
        .join("");

      res.end(`
                <!DOCTYPE html>
                <html>
                <head><title>Directory Listing</title></head>
                <body>
                    <h1>Directory: ${requestPath}</h1>
                    <ul>${fileList}</ul>
                    <p><a href="/">Go back to index</a></p>
                </body>
                </html>
            `);
    }
    return;
  }

  // Serve the file
  serveFile(filePath, res);
});

function serveFile(filePath, res) {
  const mimeType = getMimeType(filePath);
  const stream = fs.createReadStream(filePath);

  res.setHeader("Content-Type", mimeType);

  stream.on("error", (error) => {
    console.error("Error reading file:", error);
    res.writeHead(500, { "Content-Type": "text/plain" });
    res.end("Internal Server Error");
  });

  stream.pipe(res);
}

// Start server
server.listen(PORT, () => {
  console.log(`🚀 Test web server running at http://localhost:${PORT}`);
  console.log(`📁 Serving static files from: ${STATIC_DIR}`);
  console.log(`🧪 Available test forms:`);
  console.log(`   - Basic Form: http://localhost:${PORT}/basic-form.html`);
  console.log(`   - Complex Form: http://localhost:${PORT}/complex-form.html`);
  console.log(`   - Components: http://localhost:${PORT}/components.html`);
  console.log(`   - Index: http://localhost:${PORT}/`);
  console.log(`\n💡 Press Ctrl+C to stop the server`);
});

// Graceful shutdown
process.on("SIGINT", () => {
  console.log("\n🛑 Shutting down server...");
  server.close(() => {
    console.log("✅ Server stopped");
    process.exit(0);
  });
});

process.on("SIGTERM", () => {
  console.log("\n🛑 Shutting down server...");
  server.close(() => {
    console.log("✅ Server stopped");
    process.exit(0);
  });
});
