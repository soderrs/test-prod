CREATE TABLE posts (
    id TEXT,
    content TEXT,
    author TEXT,
    tags JSON,
    created_at TEXT,
    likes_count INTEGER,
    dislikes_count INTEGER,
    liked_users JSON,
    disliked_users JSON
);
