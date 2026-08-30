import React, { useEffect, useState } from "react";

const api = async (path, options) => {
  const response = await fetch(path, options);
  if (!response.ok) {
    const detail = await response.json().catch(() => ({}));
    throw new Error(detail.error ?? response.statusText);
  }
  return response.json();
};

export default function App() {
  const [authors, setAuthors] = useState([]);
  const [notes, setNotes] = useState([]);
  const [stats, setStats] = useState({ authors: 0, notes: 0, tags: 0 });
  const [form, setForm] = useState({ authorId: "", title: "", body: "", tags: "" });
  const [failure, setFailure] = useState("");

  const reload = async () => {
    const [loadedNotes, loadedStats] = await Promise.all([
      api("/api/notes"),
      api("/api/stats")
    ]);
    setNotes(loadedNotes);
    setStats(loadedStats);
  };

  useEffect(() => {
    api("/api/authors")
      .then((loaded) => {
        setAuthors(loaded);
        setForm((current) => ({ ...current, authorId: String(loaded[0]?.id ?? "") }));
      })
      .then(reload)
      .catch((error) => setFailure(error.message));
  }, []);

  const run = (work) => work().then(reload).catch((error) => setFailure(error.message));

  const submit = (event) => {
    event.preventDefault();
    setFailure("");
    run(() =>
      api("/api/notes", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify(form)
      }).then(() => setForm({ ...form, title: "", body: "", tags: "" }))
    );
  };

  const seed = () =>
    run(() =>
      api("/api/seed", {
        method: "POST",
        headers: { "content-type": "application/json" },
        body: JSON.stringify({ rows: 5000 })
      })
    );

  return (
    <main>
      <header>
        <h1>Notes</h1>
        <p>
          {stats.notes} notes · {stats.authors} authors · {stats.tags} tags · stored in
          <code>app/app.db</code>
        </p>
      </header>

      {failure && <p className="failure">{failure}</p>}

      <form onSubmit={submit}>
        <select
          value={form.authorId}
          onChange={(event) => setForm({ ...form, authorId: event.target.value })}
        >
          {authors.map((author) => (
            <option key={author.id} value={author.id}>
              {author.name}
            </option>
          ))}
        </select>
        <input
          placeholder="title"
          value={form.title}
          onChange={(event) => setForm({ ...form, title: event.target.value })}
        />
        <input
          placeholder="tags, comma separated"
          value={form.tags}
          onChange={(event) => setForm({ ...form, tags: event.target.value })}
        />
        <textarea
          placeholder="body"
          rows={3}
          value={form.body}
          onChange={(event) => setForm({ ...form, body: event.target.value })}
        />
        <div className="actions">
          <button type="submit">add note</button>
          <button type="button" className="ghost" onClick={seed}>
            seed 5,000 rows
          </button>
        </div>
      </form>

      <ul>
        {notes.slice(0, 100).map((note) => (
          <li key={note.id} className={note.pinned ? "pinned" : ""}>
            <div>
              <strong>{note.title}</strong>
              <span className="meta">
                {note.author} · {note.created_at}
                {note.tags ? ` · ${note.tags}` : ""}
              </span>
              {note.body && <p>{note.body}</p>}
            </div>
            <div className="row-actions">
              <button onClick={() => run(() => api(`/api/notes/${note.id}`, { method: "PATCH" }))}>
                {note.pinned ? "unpin" : "pin"}
              </button>
              <button
                className="ghost"
                onClick={() => run(() => api(`/api/notes/${note.id}`, { method: "DELETE" }))}
              >
                delete
              </button>
            </div>
          </li>
        ))}
      </ul>
      {notes.length > 100 && <p className="meta">showing the newest 100 of {notes.length}</p>}
    </main>
  );
}
