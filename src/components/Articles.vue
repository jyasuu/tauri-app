<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

// Define the Article type
interface Article {
  id: number;
  title: string;
  url: string;
  image_url: string;
  news_site: string;
  summary: string;
  published_at: string;
  updated_at: string;
  featured: boolean;
  launches: Array<any>;
  events: Array<any>;
}

// Define the ApiResponse type
interface ApiResponse {
  count: number;
  next: string | null;
  previous: string | null;
  results: Article[];
}

const articles = ref<Article[]>([]);
const count = ref(0);
const next = ref<string | null>(null);
const previous = ref<string | null>(null);
const limit = ref(10);
const offset = ref(0);

const fetchArticles = async (currentOffset = 0) => {
  try {
    const data = await invoke<ApiResponse>('fetch_articles', { limit: limit.value, offset: currentOffset });

    count.value = data.count;
    next.value = data.next;
    previous.value = data.previous;
    articles.value = data.results;
    offset.value = currentOffset;
  } catch (error) {
    console.error('Error fetching articles:', error);
  }
};

onMounted(() => {
  fetchArticles();
});
</script>


<template>
  <div class="p-4">
    <h1 class="text-xl font-bold mb-4">Articles</h1>
    <p>Total Articles: {{ count }}</p>
    <div class="mb-4">
      <button
        v-if="previous"
        @click="fetchArticles(offset - limit)"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
      >
        Previous
      </button>
      <button
        v-if="next"
        @click="fetchArticles(offset + limit)"
        class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded ml-2"
      >
        Next
      </button>
    </div>
    <ul class="space-y-4 overflow-y-auto max-h-96">
      <li v-for="article in articles" :key="article.id" class="border p-4 rounded-lg">
        <h2 class="text-lg font-semibold mb-2">{{ article.title }}</h2>
        <a :href="article.url" target="_blank" class="text-blue-600 hover:underline">Read More</a>
        <img
          :src="article.image_url"
          alt="Article Image"
          class="w-full h-48 object-cover mt-2 rounded-lg"
        />
        <p class="mt-2 text-gray-700">{{ article.summary }}</p>
      </li>
    </ul>
  </div>
</template>

  