# Test Web Server

This directory contains a simple web server and static HTML forms for testing the Leptos Forms library with Playwright E2E tests.

## Overview

The test web server provides a collection of HTML forms that simulate the functionality that would be expected from the Leptos Forms library. This allows us to test form interactions, validation, and user experience without needing to run the actual Leptos applications.

## Files

- **`server.js`** - Simple Node.js HTTP server that serves static files
- **`index.html`** - Test suite index page with navigation to all test forms
- **`basic-form.html`** - Simple login form with basic validation
- **`complex-form.html`** - Multi-step registration form with advanced features
- **`components.html`** - Component showcase with all form input types

## Running the Server

### Option 1: Using npm script
```bash
pnpm run test:server
```

### Option 2: Direct execution
```bash
node tests/e2e/web-server/server.js
```

### Option 3: With custom port
```bash
PORT=8080 node tests/e2e/web-server/server.js
```

## Server Features

- **Static File Serving**: Serves HTML, CSS, JS, and other static files
- **CORS Support**: Configured for cross-origin testing
- **Security**: Prevents path traversal attacks
- **Directory Listing**: Shows available files when accessing directories
- **Error Handling**: Custom 404 and error pages
- **Logging**: Request logging with timestamps

## Test Forms

### Basic Form (`/basic-form.html`)
- Simple login form with username/password
- Real-time validation
- Error message display
- Success feedback
- Form reset functionality

### Complex Form (`/complex-form.html`)
- Multi-step registration process
- Conditional field rendering
- Field arrays (add/remove items)
- File upload handling
- Step-by-step validation
- Progress indicators

### Component Showcase (`/components.html`)
- All HTML input types
- Checkbox and radio groups
- Select dropdowns and textareas
- File upload components
- Conditional field rendering
- Field array manipulation
- Form validation and submission

## Integration with Playwright

The web server is configured to work with Playwright E2E tests:

1. **Web Server Configuration**: Playwright automatically starts the server before running tests
2. **Base URL**: Tests use `http://localhost:3000` as the base URL
3. **Test IDs**: All forms include comprehensive `data-testid` attributes
4. **Global Setup**: Server readiness is verified before tests begin

## Test Coverage

The forms cover the following scenarios:

- **Form Rendering**: Basic HTML structure and styling
- **User Input**: Text, number, email, password, date, time inputs
- **Form Controls**: Checkboxes, radio buttons, selects, textareas
- **File Handling**: File uploads with type restrictions
- **Validation**: Client-side validation with error messages
- **Conditional Logic**: Fields that appear/disappear based on conditions
- **Dynamic Content**: Adding/removing form sections
- **Form Submission**: Data collection and success handling
- **Accessibility**: Proper labels, ARIA attributes, keyboard navigation
- **Responsive Design**: Mobile-friendly layouts and interactions

## Development

### Adding New Test Forms
1. Create a new HTML file in this directory
2. Include comprehensive `data-testid` attributes
3. Add navigation link to `index.html`
4. Update this README with form description

### Modifying the Server
- The server is a simple Node.js HTTP server
- Supports custom MIME types for different file extensions
- Includes security measures against path traversal
- Can be extended with additional middleware if needed

### Testing Locally
1. Start the server: `pnpm run test:server`
2. Open `http://localhost:3000` in your browser
3. Navigate through the test forms
4. Verify all functionality works as expected

## Troubleshooting

### Port Already in Use
If port 3000 is already in use, set a different port:
```bash
PORT=8080 pnpm run test:server
```

### File Not Found
Ensure all HTML files are in the correct directory and have proper permissions.

### CORS Issues
The server includes CORS headers for testing. If you encounter issues, check that the server is running and accessible.

### Playwright Connection Issues
If Playwright can't connect to the server:
1. Verify the server is running on the expected port
2. Check that the `webServer` configuration in `playwright.config.ts` is correct
3. Ensure no firewall or network restrictions are blocking the connection

## Next Steps

Once the test web server is working with Playwright:

1. **Run E2E Tests**: Execute `pnpm run test:e2e` to run all tests
2. **View Reports**: Use `pnpm run test:e2e:show-report` to see test results
3. **Debug Tests**: Use `pnpm run test:e2e:debug` for interactive debugging
4. **Generate Tests**: Use `pnpm run test:e2e:codegen` to create new tests

This setup provides a solid foundation for testing form functionality and can be extended as the Leptos Forms library evolves.
