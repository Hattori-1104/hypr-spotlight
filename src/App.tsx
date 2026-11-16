import { invoke } from "@tauri-apps/api/core"
import { useEffect, useState } from "react"
import "./style.css"

interface DesktopEntry {
	name: string
	path: string
}

export default function App() {
	const [entries, setEntries] = useState<DesktopEntry[]>([])
	const [error, setError] = useState<string | null>(null)

	useEffect(() => {
		invoke<DesktopEntry[]>("list_cached")
			.then(setEntries)
			.catch((err) => setError(String(err)))
	}, [])

	return (
		<main>
			<h1>Applications ({entries.length})</h1>
			{error && <p role="alert">{error}</p>}

			<div className="p-4 border flex flex-col items-stretch gap-4">
				<input className="border" type="text" />
				<div className="max-h-64 min-h-min overflow-y-auto">
					{entries.map((entry) => (
						<div key={entry.path}>{entry.name}</div>
					))}
				</div>
			</div>
		</main>
	)
}
