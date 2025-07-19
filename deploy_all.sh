#!/usr/bin/env bash
set -eEuo pipefail

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo -e "${GREEN}Starting Orbit local deployment...${NC}"

# Function to check if dfx is running
check_dfx_running() {
    if dfx ping &>/dev/null; then
        return 0
    else
        return 1
    fi
}

# Function to clean up existing state
cleanup_state() {
    echo -e "${YELLOW}Cleaning up existing state...${NC}"
    
    # Stop all canisters if dfx is running
    if check_dfx_running; then
        echo "Stopping existing canisters..."
        dfx canister stop --all 2>/dev/null || true
        dfx canister delete --all --no-withdrawal 2>/dev/null || true
    fi
    
    # Remove .dfx directory for clean state
    if [ -d ".dfx" ]; then
        echo "Removing .dfx directory..."
        rm -rf .dfx
    fi
}

# Function to start dfx
start_dfx() {
    if check_dfx_running; then
        echo -e "${YELLOW}dfx is already running${NC}"
        # Check if we need to restart for clean state
        read -p "Do you want to restart dfx with clean state? (y/N) " -n 1 -r
        echo
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            echo "Stopping dfx..."
            dfx stop
            cleanup_state
            echo "Starting dfx with clean state..."
            dfx start --clean --background
            # Wait for dfx to be ready
            sleep 3
        fi
    else
        cleanup_state
        echo -e "${GREEN}Starting dfx...${NC}"
        dfx start --clean --background
        # Wait for dfx to be ready
        sleep 3
    fi
}

# Function to deploy all components
deploy_all() {
    echo -e "${GREEN}Deploying all Orbit components...${NC}"
    
    # Use the orbit script with --init flag
    if [ -x "./orbit" ]; then
        ./orbit --init
    else
        echo -e "${RED}Error: orbit script not found or not executable${NC}"
        exit 1
    fi
}

# Main execution
main() {
    echo -e "${GREEN}=== Orbit Local Deployment Script ===${NC}"
    echo -e "${YELLOW}WARNING: This is a LOCAL DEVELOPMENT ONLY deployment${NC}"
    echo -e "${YELLOW}DO NOT use this for mainnet deployment${NC}"
    echo
    
    # Check dependencies
    if ! command -v dfx &> /dev/null; then
        echo -e "${RED}Error: dfx is not installed${NC}"
        exit 1
    fi
    
    if ! command -v node &> /dev/null; then
        echo -e "${RED}Error: node is not installed${NC}"
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}Error: cargo is not installed${NC}"
        exit 1
    fi
    
    # Start deployment process
    start_dfx
    
    # Verify dfx is running
    if ! check_dfx_running; then
        echo -e "${RED}Error: dfx failed to start${NC}"
        exit 1
    fi
    
    # Deploy all components
    deploy_all
    
    echo
    echo -e "${GREEN}=== Deployment Complete ===${NC}"
    echo -e "${GREEN}Wallet UI available at:${NC} http://werw6-ayaaa-aaaaa-774aa-cai.localhost:4943/"
    echo -e "${GREEN}Control Panel Candid UI:${NC} http://127.0.0.1:4943/?canisterId=lxzze-o7777-77777-aaaaa-cai&id=wdqqk-naaaa-aaaaa-774aq-cai"
    echo
    echo -e "${YELLOW}Note: This deployment uses management canister instead of CMC for local development${NC}"
}

# Run main function
main "$@"