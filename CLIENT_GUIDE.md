# RustyTunnel VPN Client - Development & Deployment Guide

## Overview

The RustyTunnel VPN Client is a modern desktop application built with Tauri and React, providing a user-friendly interface for connecting to VPN servers.

## Features

### User Interface
- **Login Page** - Secure authentication with username/password
- **Dashboard** - Main VPN control interface
- **Server Selection** - Browse and select from available VPN servers
- **Real-time Statistics** - Monitor upload/download speeds and data usage
- **Settings Panel** - Customize application behavior
- **Status Indicators** - Connection status, server load, and ping

### Functionality
- ✅ User authentication
- ✅ VPN server listing with real-time ping
- ✅ One-click connection/disconnection
- ✅ Real-time speed monitoring
- ✅ Data usage tracking
- ✅ Connection time display
- ✅ IP address display
- ✅ Protocol selection (WireGuard, OpenVPN, IKEv2)
- ✅ DNS configuration
- ✅ Theme customization
- ✅ Multi-language support

## Project Structure

```
client/src-tauri/
├── src/                          # Rust backend
│   ├── main.rs                  # Tauri application entry
│   ├── commands.rs              # Tauri command handlers
│   ├── models.rs                # Data models
│   ├── state.rs                 # Application state management
│   └── error.rs                 # Error handling
│
├── frontend/                     # React frontend
│   ├── src/
│   │   ├── pages/
│   │   │   ├── LoginPage.jsx    # Login interface
│   │   │   └── DashboardPage.jsx # Main dashboard
│   │   ├── components/
│   │   │   ├── NodeSelector.jsx  # Server selection
│   │   │   ├── StatsDisplay.jsx  # Statistics display
│   │   │   └── SettingsPanel.jsx # Settings dialog
│   │   ├── App.jsx              # Root component
│   │   ├── api.js               # Tauri API wrapper
│   │   ├── main.jsx             # React entry point
│   │   └── index.css            # Global styles
│   ├── index.html
│   ├── vite.config.js
│   ├── tailwind.config.js
│   ├── postcss.config.js
│   └── package.json
│
├── src/
│   ├── main.rs                  # Rust main
│   ├── commands.rs              # Command implementations
│   ├── models.rs                # Data structures
│   ├── state.rs                 # State management
│   └── error.rs                 # Error types
│
├── build.rs                     # Tauri build script
├── tauri.conf.json              # Tauri configuration
├── Cargo.toml                   # Rust dependencies
└── frontend/package.json        # Node dependencies
```

## Development Setup

### Prerequisites
- Rust 1.70+ (with cargo)
- Node.js 16+
- npm or yarn

### Installation

1. Clone the repository:
```bash
git clone https://github.com/chenjiafa9/RustyTunnel.git
cd RustyTunnel/client/src-tauri
```

2. Install dependencies:
```bash
# Install Rust dependencies
cargo fetch

# Install Node dependencies
cd frontend
npm install
cd ..
```

### Running in Development Mode

Terminal 1 - Start the frontend dev server:
```bash
cd frontend
npm run dev
```

Terminal 2 - Start the Tauri application:
```bash
cargo tauri dev
```

The application will open with hot-reload enabled.

### Building for Production

```bash
# Build frontend
cd frontend
npm run build
cd ..

# Build Tauri application
cargo tauri build
```

The executable will be generated in `src-tauri/target/release/`.

## Architecture

### Backend (Rust)

The Rust backend handles:
- **State Management** - User session, connection stats, settings
- **Command Handlers** - Tauri commands for frontend communication
- **Data Models** - User, VPN nodes, connection stats
- **Error Handling** - Structured error responses

### Frontend (React)

The React frontend provides:
- **Login Page** - User authentication
- **Dashboard** - Main VPN interface
- **Components** - Reusable UI components
- **API Integration** - Tauri command invocation

### Data Flow

```
Frontend (React)
    ↓
API Layer (Tauri Commands)
    ↓
Backend (Rust)
    ↓
State Management
    ↓
Response to Frontend
```

## API Commands

### Authentication
- `login(username, password)` - User login
- `logout()` - User logout
- `get_current_user()` - Get current user info

### VPN Operations
- `get_vpn_nodes()` - Get available servers
- `start_connection(node_id)` - Connect to server
- `stop_connection()` - Disconnect from server
- `test_node_connection(node_id)` - Test ping to server

### Statistics
- `get_connection_stats()` - Get current stats
- `update_connection_stats(stats)` - Update stats

### Settings
- `get_settings()` - Get app settings
- `update_settings(settings)` - Save app settings

## Data Models

### User
```rust
struct UserInfo {
    id: String,
    username: String,
    email: String,
}
```

### VPN Node
```rust
struct VpnNode {
    id: String,
    name: String,
    country: String,
    city: String,
    protocol: String,
    endpoint: String,
    port: u16,
    public_key: String,
    ping: Option<u32>,
    load: Option<f32>,
}
```

### Connection Stats
```rust
struct ConnectionStats {
    status: ConnectionStatus,
    connected_node: Option<String>,
    upload_speed: u64,
    download_speed: u64,
    total_uploaded: u64,
    total_downloaded: u64,
    connection_time: u64,
    ip_address: Option<String>,
}
```

### App Settings
```rust
struct AppSettings {
    theme: String,
    language: String,
    auto_start: bool,
    minimize_to_tray: bool,
    default_protocol: String,
    dns_servers: Vec<String>,
}
```

## Styling

The application uses Tailwind CSS with custom components:

- `.btn` - Button base styles
- `.btn-primary` - Primary button
- `.btn-secondary` - Secondary button
- `.btn-danger` - Danger button
- `.card` - Card container
- `.input` - Input field
- `.badge` - Status badge

## Testing

### Demo Credentials
- Username: `demo`
- Password: `demo`

### Demo Servers
- New York (US)
- London (UK)
- Tokyo (JP)
- Singapore (SG)

## Deployment

### Windows

1. Build the application:
```bash
cargo tauri build
```

2. The installer will be generated at:
```
src-tauri/target/release/bundle/msi/
```

3. Run the MSI installer to install the application.

### Configuration

Application settings are stored in the system's application data directory:
- Windows: `%APPDATA%\RustyTunnel VPN\`

## Troubleshooting

### Application won't start
- Ensure all dependencies are installed
- Check Rust version: `rustc --version`
- Check Node version: `node --version`

### Hot reload not working
- Ensure frontend dev server is running on port 5173
- Check `tauri.conf.json` devPath setting

### Build fails
- Clear cache: `cargo clean`
- Update dependencies: `cargo update`
- Reinstall Node modules: `rm -rf frontend/node_modules && npm install`

### Connection issues
- Verify server is running
- Check firewall settings
- Test with demo credentials first

## Performance Optimization

### Frontend
- React lazy loading for pages
- Component memoization
- CSS optimization with Tailwind

### Backend
- Async/await for non-blocking operations
- Efficient state management
- Minimal memory footprint

## Security Considerations

- Passwords are never stored locally
- All communication uses Tauri's secure bridge
- Tokens are managed in-memory
- No sensitive data in logs

## Future Enhancements

- [ ] Tray icon with quick connect
- [ ] Auto-reconnect on network change
- [ ] Kill switch functionality
- [ ] Split tunneling
- [ ] Custom server support
- [ ] VPN protocol auto-selection
- [ ] Advanced firewall rules
- [ ] System notifications
- [ ] Dark/Light theme toggle
- [ ] Multi-language support

## Contributing

To contribute to the client development:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## License

MIT License - See LICENSE file for details

## Support

For issues and questions:
- Open an issue on GitHub
- Check existing documentation
- Review troubleshooting section
