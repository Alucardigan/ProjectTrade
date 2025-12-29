import React from 'react';

const LoginPage: React.FC = () => {
    return (
        <div className="flex flex-col items-center justify-center h-screen bg-gray-100">
            <div className="p-8 bg-white rounded-lg shadow-md">
                <h1 className="mb-6 text-2xl font-bold text-center text-gray-800">Login</h1>
                <form action="http://localhost:3000/login_user" method="post">
                    <button
                        type="submit"
                        className="px-6 py-2 font-semibold text-white transition-colors duration-200 bg-blue-600 rounded hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    >
                        Login with Auth0
                    </button>
                </form>
            </div>
        </div>
    );
};

export default LoginPage;
