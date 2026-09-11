export interface Reaction {
    id: number;
    message_id: number;
    author: string;
    emoji: string;
}

export interface ReactionGroup {
    emoji: string;
    count: number;
    reactedByMe: boolean;
}

export interface Message {
    id: number;
    author: string;
    body: string;
    created_at: string;
    sticker: string | null;
    reactions: ReactionGroup[];
}
