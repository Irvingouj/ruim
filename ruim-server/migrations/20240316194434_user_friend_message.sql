CREATE TABLE messages (
    message_id SERIAL PRIMARY KEY,
    sender_id UUID REFERENCES users(user_id),
    receiver_id UUID REFERENCES users(user_id),
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE friendships (
    friendship_id SERIAL PRIMARY KEY,
    user1_id UUID REFERENCES users(user_id) NOT NULL,
    user2_id UUID REFERENCES users(user_id) NOT NULL,
    status smallint NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE friend_applications (
    application_id SERIAL PRIMARY KEY,
    sender_id UUID REFERENCES users(user_id) NOT NULL,
    receiver_id UUID REFERENCES users(user_id) NOT NULL,
    status smallint NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
