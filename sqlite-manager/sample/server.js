import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const PORT = 7777;
const db = new Database("sample.db", { create: true });
db.exec(readFileSync("schema.sql", "utf8"));
db.exec("PRAGMA foreign_keys = ON");

if (db.query("SELECT count(*) AS total FROM authors").get().total === 0) {
  const insert = db.prepare("INSERT INTO authors (name, email) VALUES (?, ?)");
  insert.run("Ada", "ada@notes.dev");
  insert.run("Grace", "grace@notes.dev");
  insert.run("Alan", "alan@notes.dev");
}

const json = (body, status = 200) =>
  new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json" }
  });

const listNotes = () =>
  db
    .query(
      `SELECT n.id, n.title, n.body, n.pinned, n.created_at, a.name AS author,
              (SELECT group_concat(t.label, ', ')
               FROM note_tags nt JOIN tags t ON t.id = nt.tag_id
               WHERE nt.note_id = n.id) AS tags
       FROM notes n JOIN authors a ON a.id = n.author_id
       ORDER BY n.pinned DESC, n.id DESC`
    )
    .all();

const attachTags = db.transaction((noteId, labels) => {
  const findTag = db.prepare("SELECT id FROM tags WHERE label = ?");
  const addTag = db.prepare("INSERT INTO tags (label) VALUES (?)");
  const link = db.prepare("INSERT OR IGNORE INTO note_tags (note_id, tag_id) VALUES (?, ?)");
  for (const label of labels) {
    const existing = findTag.get(label);
    const id = existing ? existing.id : addTag.run(label).lastInsertRowid;
    link.run(noteId, id);
  }
});

const routes = {
  "GET /api/authors": () => json(db.query("SELECT id, name, email FROM authors ORDER BY name").all()),

  "GET /api/notes": () => json(listNotes()),

  "GET /api/stats": () =>
    json({
      authors: db.query("SELECT count(*) AS n FROM authors").get().n,
      notes: db.query("SELECT count(*) AS n FROM notes").get().n,
      tags: db.query("SELECT count(*) AS n FROM tags").get().n
    }),

  "POST /api/notes": async (request) => {
    const { authorId, title, body, tags } = await request.json();
    if (!authorId || !title?.trim()) {
      return json({ error: "authorId and title are required" }, 400);
    }
    const result = db
      .prepare("INSERT INTO notes (author_id, title, body) VALUES (?, ?, ?)")
      .run(Number(authorId), title.trim(), body ?? "");
    const labels = (tags ?? "")
      .split(",")
      .map((label) => label.trim())
      .filter(Boolean);
    if (labels.length > 0) {
      attachTags(result.lastInsertRowid, labels);
    }
    return json(listNotes(), 201);
  },

  "POST /api/seed": async (request) => {
    const { rows } = await request.json();
    const total = Math.min(Math.max(Number(rows) || 0, 1), 50000);
    const authors = db.query("SELECT id FROM authors").all().map((row) => row.id);
    const insert = db.prepare("INSERT INTO notes (author_id, title, body) VALUES (?, ?, ?)");
    const seed = db.transaction((count) => {
      for (let index = 0; index < count; index += 1) {
        const author = authors[index % authors.length];
        insert.run(author, `note ${index + 1}`, `body for note ${index + 1}`);
      }
    });
    seed(total);
    return json({ inserted: total });
  }
};

const server = Bun.serve({
  port: PORT,
  async fetch(request) {
    const url = new URL(request.url);
    const key = `${request.method} ${url.pathname}`;

    if (routes[key]) {
      return routes[key](request);
    }
    if (request.method === "DELETE" && url.pathname.startsWith("/api/notes/")) {
      const id = Number(url.pathname.split("/").pop());
      db.prepare("DELETE FROM notes WHERE id = ?").run(id);
      return json(listNotes());
    }
    if (request.method === "PATCH" && url.pathname.startsWith("/api/notes/")) {
      const id = Number(url.pathname.split("/").pop());
      db.prepare("UPDATE notes SET pinned = 1 - pinned WHERE id = ?").run(id);
      return json(listNotes());
    }
    return json({ error: "not found" }, 404);
  }
});

console.log(`notes api on http://localhost:${server.port} writing to sample.db`);
