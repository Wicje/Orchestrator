-- Frameworks table
CREATE TABLE frameworks (
    id TEXT PRIMARY KEY,
    language TEXT NOT NULL,
    base_scaffold_command TEXT,   -- e.g. "npm create vite@latest"
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Features table
CREATE TABLE features (
    id TEXT PRIMARY KEY,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Framework-Feature relationships (many-to-many)
CREATE TABLE framework_features (
    framework_id TEXT NOT NULL,
    feature_id TEXT NOT NULL,
    required BOOLEAN DEFAULT false,   -- if true, feature is always included
    FOREIGN KEY (framework_id) REFERENCES frameworks(id),
    FOREIGN KEY (feature_id) REFERENCES features(id),
    PRIMARY KEY (framework_id, feature_id)
);

-- Dependencies for each feature (can be framework-specific or global)
CREATE TABLE dependencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    framework_id TEXT,                -- NULL means applies to all frameworks
    feature_id TEXT NOT NULL,
    package_name TEXT NOT NULL,
    version_constraint TEXT NOT NULL,
    is_dev BOOLEAN DEFAULT false,
    FOREIGN KEY (framework_id) REFERENCES frameworks(id),
    FOREIGN KEY (feature_id) REFERENCES features(id)
);

-- Configuration mutations (e.g. add a plugin to vite.config.js)
CREATE TABLE config_mutations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    framework_id TEXT NOT NULL,
    feature_id TEXT NOT NULL,
    file_path TEXT NOT NULL,          -- relative to project root
    mutation_type TEXT NOT NULL,      -- "json_merge", "text_append", etc.
    content TEXT NOT NULL,            -- the data to merge/append
    FOREIGN KEY (framework_id) REFERENCES frameworks(id),
    FOREIGN KEY (feature_id) REFERENCES features(id)
);

-- Seed data: React framework
INSERT INTO frameworks (id, language, base_scaffold_command) VALUES
    ('react', 'javascript', 'npm create vite@latest . -- --template react'),
    ('react-ts', 'typescript', 'npm create vite@latest . -- --template react-ts');

INSERT INTO features (id, description) VALUES
    ('typescript', 'TypeScript support'),
    ('tailwind', 'Tailwind CSS'),
    ('eslint', 'ESLint configuration'),
    ('router', 'React Router');

-- React with TypeScript (react-ts already includes TS, so we mark it as required)
INSERT INTO framework_features (framework_id, feature_id, required) VALUES
    ('react', 'typescript', false),
    ('react-ts', 'typescript', true),
    ('react', 'tailwind', false),
    ('react-ts', 'tailwind', false),
    ('react', 'eslint', false),
    ('react-ts', 'eslint', false),
    ('react', 'router', false),
    ('react-ts', 'router', false);

-- Dependencies for features (simplified)
INSERT INTO dependencies (framework_id, feature_id, package_name, version_constraint, is_dev) VALUES
    (NULL, 'typescript', 'typescript', '^5.0', true),
    ('react', 'tailwind', 'tailwindcss', '^3.0', true),
    ('react-ts', 'tailwind', 'tailwindcss', '^3.0', true),
    (NULL, 'eslint', 'eslint', '^8.0', true),
    (NULL, 'eslint', 'eslint-plugin-react', '^7.0', true),
    ('react', 'router', 'react-router-dom', '^6.0', false),
    ('react-ts', 'router', 'react-router-dom', '^6.0', false);

-- Config mutations (example: add tailwind to vite config)
INSERT INTO config_mutations (framework_id, feature_id, file_path, mutation_type, content) VALUES
    ('react', 'tailwind', 'vite.config.js', 'text_append', 'import tailwindcss from "tailwindcss";\n...'),
    ('react-ts', 'tailwind', 'vite.config.ts', 'text_append', 'import tailwindcss from "tailwindcss";\n...');
