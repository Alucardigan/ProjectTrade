import React from 'react';
import { Button } from "@/components/retroui/Button";

const LoginPage: React.FC = () => {
    return (
        <div className="flex flex-col items-center justify-center h-screen bg-red-500">
            <div className="p-8 bg-white rounded-lg shadow-md">
                <h1 className="mb-6 text-2xl font-bold text-center text-gray-800">Login</h1>
                <form action="http://localhost:3000/auth/login" method="post">
                    <Button
                        type="submit"
                        className="w-full"
                    >
                        Login with Auth0
                    </Button>
                </form>
            </div>
        </div>
    );
};

export default LoginPage;
